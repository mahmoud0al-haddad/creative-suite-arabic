//! Optional OFL Arabic faces from the separately pinned craft-fonts build input.
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn entries(dir: &Path) -> Result<String, String> {
    let manifest = dir.join("fonts/manifest.txt");
    println!("cargo::rerun-if-changed={}", manifest.display());
    let text = std::fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
    let mut output = String::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty() && !line.starts_with('#')) {
        let fields: Vec<&str> = line.split(" | ").map(str::trim).collect();
        let [family, style, file, scripts, ..] = fields.as_slice() else {
            return Err(format!("malformed font manifest row: {line}"));
        };
        if !scripts.split(',').any(|script| script.trim() == "Arab") {
            continue;
        }
        let path = dir.join(file).canonicalize().map_err(|e| e.to_string())?;
        println!("cargo::rerun-if-changed={}", path.display());
        let _ = writeln!(output, "CraftFont {{ family: {family:?}, style: {style:?}, bytes: include_bytes!({:?}) }},", path.display().to_string());
    }
    if output.is_empty() {
        return Err("the font manifest has no Arabic face".into());
    }
    Ok(output)
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_DIR");
    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_REQUIRED");
    let mut source = String::from("pub static CRAFT_FONTS: &[CraftFont] = &[\n");
    if let Some(dir) = std::env::var_os("CRAFT_FONTS_DIR").map(PathBuf::from) {
        match entries(&dir) {
            Ok(fonts) => source.push_str(&fonts),
            Err(error) if std::env::var_os("CRAFT_FONTS_REQUIRED").is_some() => println!("cargo::error={error}"),
            Err(error) => println!("cargo::warning=building without optional Arabic faces: {error}"),
        }
    } else if std::env::var_os("CRAFT_FONTS_REQUIRED").is_some() {
        println!("cargo::error=CRAFT_FONTS_REQUIRED is set but CRAFT_FONTS_DIR is absent");
    }
    source.push_str("];\n");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("craft_fonts.rs");
    if let Err(error) = std::fs::write(output, source) {
        println!("cargo::error=writing Arabic font table: {error}");
    }
}
