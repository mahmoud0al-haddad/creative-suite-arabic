//! Run the same workspace/path-dependency Rust targets in bounded Windows command lines.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

/// Workspace packages and their explicit local path dependencies; registry patches keep their
/// upstream formatting, just as with cargo-fmt's recursive path-dependency selection.
fn local_targets(metadata: &Value) -> Result<BTreeMap<String, Vec<PathBuf>>, String> {
    let packages = metadata.get("packages").and_then(Value::as_array).ok_or("metadata has no packages")?;
    let mut selected: BTreeSet<String> = metadata
        .get("workspace_members")
        .and_then(Value::as_array)
        .ok_or("metadata has no workspace members")?
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect();
    if selected.is_empty() {
        return Err("workspace has no formatting targets".into());
    }
    loop {
        let mut added = BTreeSet::new();
        for package in packages {
            let Some(id) = package.get("id").and_then(Value::as_str) else { return Err("package has no id".into()) };
            if !selected.contains(id) {
                continue;
            }
            let dependencies = package.get("dependencies").and_then(Value::as_array).ok_or("package has no dependency list")?;
            for dependency in dependencies {
                let Some(path) = dependency.get("path").and_then(Value::as_str) else { continue };
                let target = packages
                    .iter()
                    .find(|candidate| candidate.get("manifest_path").and_then(Value::as_str).and_then(|p| Path::new(p).parent()) == Some(Path::new(path)));
                let id = target.and_then(|p| p.get("id")).and_then(Value::as_str).ok_or_else(|| format!("local formatting dependency not found: {path}"))?;
                if !selected.contains(id) {
                    added.insert(id.to_string());
                }
            }
        }
        if added.is_empty() {
            break;
        }
        selected.extend(added);
    }
    let mut grouped: BTreeMap<String, BTreeSet<PathBuf>> = BTreeMap::new();
    for id in selected {
        let package = packages.iter().find(|p| p.get("id").and_then(Value::as_str) == Some(&id)).ok_or("workspace member not found")?;
        let targets = package.get("targets").and_then(Value::as_array).ok_or("package has no targets")?;
        for target in targets {
            let source = target.get("src_path").and_then(Value::as_str).ok_or("target has no source path")?;
            if !source.ends_with(".rs") {
                continue;
            }
            let edition = target.get("edition").or_else(|| package.get("edition")).and_then(Value::as_str).ok_or("target has no edition")?;
            grouped.entry(edition.to_string()).or_default().insert(PathBuf::from(source));
        }
    }
    Ok(grouped.into_iter().map(|(edition, files)| (edition, files.into_iter().collect())).collect())
}

pub fn run(check: bool) -> Result<(), String> {
    if !cfg!(windows) {
        let mut command = crate::cargo();
        command.args(["fmt", "--all"]);
        if check {
            command.args(["--", "--check"]);
        }
        return crate::run(command, "cargo fmt --all");
    }
    let output = crate::cargo().args(["metadata", "--locked", "--format-version", "1"]).output().map_err(|e| format!("format metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!("format metadata failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let metadata: Value = serde_json::from_slice(&output.stdout).map_err(|e| format!("format metadata JSON: {e}"))?;
    let grouped = local_targets(&metadata)?;
    let rustfmt = std::env::var_os("RUSTFMT").filter(|value| !value.is_empty()).unwrap_or_else(|| "rustfmt".into());
    for (edition, files) in grouped {
        // Win32's process command line is limited to 32767 UTF-16 code units. The full set
        // exceeds it in long/Unicode checkouts; each batch still checks all child modules.
        for batch in files.chunks(24) {
            let mut command = Command::new(&rustfmt);
            command.current_dir(crate::root()).args(["--edition", &edition]);
            if check {
                command.arg("--check");
            }
            command.args(batch);
            crate::run(command, &format!("rustfmt edition {edition}: {} entry files{}", batch.len(), if check { " --check" } else { "" }))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn covers_workspace_and_recursive_paths_without_formatting_registry_patches() {
        let package = |id, root: &str, dependencies: Value, targets: Value| json!({"id":id,"manifest_path":format!("{root}/Cargo.toml"),"edition":"2024","dependencies":dependencies,"targets":targets});
        let target = |source: &str| json!({"src_path":source,"edition":"2024"});
        let metadata = json!({"workspace_members":["app","tool"],"packages":[
            package("app","workspace/app",json!([{"path":"local/dependency"},{"source":"registry","name":"patched"}]),json!([target("workspace/app/main.rs"),target("workspace/app/tests.rs")])),
            package("tool","workspace/tool",json!([]),json!([target("workspace/tool/build.rs")])),
            package("dependency","local/dependency",json!([]),json!([target("local/dependency/lib.rs")])),
            package("patched","workspace/vendor/patched",json!([]),json!([target("workspace/vendor/patched/lib.rs")]))
        ]});
        let files = local_targets(&metadata).unwrap();
        let all = files.get("2024").unwrap();
        assert_eq!(all.len(), 4);
        for expected in ["workspace/app/main.rs", "workspace/app/tests.rs", "workspace/tool/build.rs", "local/dependency/lib.rs"] {
            assert!(all.contains(&PathBuf::from(expected)), "missing {expected}");
        }
        assert!(!all.iter().any(|path| path.to_string_lossy().contains("vendor")));
    }

    #[test]
    fn missing_local_dependency_is_an_error_instead_of_skipping_a_gate() {
        let metadata = json!({"workspace_members":["app"],"packages":[{"id":"app","manifest_path":"app/Cargo.toml","edition":"2024","dependencies":[{"path":"missing"}],"targets":[]}]});
        assert!(local_targets(&metadata).is_err());
    }
}
