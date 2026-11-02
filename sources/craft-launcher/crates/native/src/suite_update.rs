//! Whole-suite updates from the user's explicitly selected independent GitHub repository.
use std::path::{Path, PathBuf};

use artcraft_launcher_core::protocol::{JobPhase, Source};
use artcraft_launcher_core::suite::{self, Metadata, SuiteRelease, SuiteStatus};
use sha2::{Digest, Sha256};

use crate::http::{Fetched, Http, Stop, Transfer};
use crate::paths::{Paths, write_atomic};

const MAX_METADATA: u64 = 128 * 1024;

pub fn installed(paths: &Paths) -> Result<Metadata, String> {
    let path = paths.data.join("bundle.json");
    if std::fs::metadata(&path).map_err(|e| e.to_string())?.len() > MAX_METADATA {
        return Err("Arabic edition metadata is too large".into());
    }
    let metadata: Metadata = serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    metadata.validate().map_err(str::to_string)?;
    Ok(metadata)
}

pub fn last_attempt(paths: &Paths) -> Option<i64> {
    std::fs::read_to_string(paths.data.join("suite-last-check.txt")).ok()?.trim().parse().ok()
}

pub fn due(last: Option<i64>, now: i64, force: bool) -> bool {
    force || last.is_none_or(|last| now.saturating_sub(last) >= suite::CHECK_INTERVAL || last > now)
}

pub fn record_attempt(paths: &Paths, at: i64) -> Result<(), String> {
    write_atomic(&paths.data.join("suite-last-check.txt"), at.to_string().as_bytes())
}

pub fn saved_status(paths: &Paths) -> Option<SuiteStatus> {
    let path = paths.data.join("suite-status.json");
    if std::fs::metadata(&path).ok()?.len() > MAX_METADATA {
        return None;
    }
    let status: SuiteStatus = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
    if let SuiteStatus::Available(update) = &status {
        update.validate(&update.metadata.tag).ok()?;
        let current = installed(paths).ok()?;
        if !update.newer_than(&current) {
            return Some(SuiteStatus::Current);
        }
    }
    Some(status)
}

pub fn save_status(paths: &Paths, status: &SuiteStatus) -> Result<(), String> {
    write_atomic(&paths.data.join("suite-status.json"), &serde_json::to_vec(status).map_err(|e| e.to_string())?)
}

/// A one-time report from the setup helper. Success also requires the requested revision on disk.
pub fn completed_install(paths: &Paths) -> Option<bool> {
    let path = paths.data.join("suite-update-result.json");
    if std::fs::metadata(&path).ok()?.len() > MAX_METADATA {
        return None;
    }
    let value: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).ok()?).ok()?;
    let revision = value.get("requested_revision")?.as_u64()?;
    let success = value.get("success")?.as_bool()?
        && value.get("exit_code").and_then(serde_json::Value::as_i64) == Some(0)
        && installed(paths).is_ok_and(|metadata| metadata.revision == revision);
    let _ = std::fs::remove_file(path);
    Some(success)
}

pub fn check(http: &Http, paths: &Paths) -> Result<(SuiteStatus, Source), String> {
    let current = installed(paths)?;
    let url = format!("https://api.github.com/repos/{}/releases/latest", suite::REPOSITORY);
    let text = match http.get_api(&url, None)? {
        Fetched::Body { text, .. } => text,
        Fetched::NotFound => return Ok((SuiteStatus::NotPublished, Source::GitHubApi)),
        Fetched::RateLimited { .. } => return Err("GitHub request limit reached".into()),
        Fetched::NotModified => return Err("unexpected cached GitHub response".into()),
    };
    let published = published_release(&text)?;
    let feed = published.assets.iter().find(|asset| asset.name == suite::FEED_NAME).ok_or("Arabic edition update manifest is missing")?;
    let body = http.get_text(&feed.url)?.ok_or("Arabic edition update manifest is unavailable")?;
    Ok((evaluate_feed(&published, &body, &current)?, Source::GitHubApi))
}

fn published_release(text: &str) -> Result<artcraft_launcher_core::Release, String> {
    let json: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if json.get("draft").and_then(serde_json::Value::as_bool) != Some(false)
        || json.get("prerelease").and_then(serde_json::Value::as_bool) != Some(false)
    {
        return Err("only a published reviewed Arabic edition is offered".into());
    }
    let published = artcraft_launcher_core::release::parse_github_release(text, suite::REPOSITORY)?;
    if !suite::valid_tag(&published.tag) {
        return Err("invalid Arabic edition tag".into());
    }
    let feed = published.assets.iter().find(|asset| asset.name == suite::FEED_NAME).ok_or("Arabic edition update manifest is missing")?;
    if feed.url != suite::asset_url(&published.tag, suite::FEED_NAME) || feed.size == 0 || feed.size > MAX_METADATA {
        return Err("invalid Arabic edition update manifest asset".into());
    }
    let _expected = feed.sha256.as_deref().filter(|hash| suite::valid_hash(hash)).ok_or("update manifest requires a published checksum")?;
    Ok(published)
}

fn evaluate_feed(published: &artcraft_launcher_core::Release, body: &str, current: &Metadata) -> Result<SuiteStatus, String> {
    current.validate().map_err(str::to_string)?;
    let feed = published.assets.iter().find(|asset| asset.name == suite::FEED_NAME).ok_or("Arabic edition update manifest is missing")?;
    let expected = feed.sha256.as_deref().filter(|hash| suite::valid_hash(hash)).ok_or("update manifest requires a published checksum")?;
    if body.len() as u64 != feed.size
        || body.len() as u64 > MAX_METADATA
        || !format!("{:x}", Sha256::digest(body.as_bytes())).eq_ignore_ascii_case(expected)
    {
        return Err("Arabic edition update manifest checksum mismatch".into());
    }
    let update: SuiteRelease = serde_json::from_str(body).map_err(|e| e.to_string())?;
    update.validate(&published.tag).map_err(str::to_string)?;
    let setup = published.assets.iter().find(|asset| asset.name == suite::SETUP_NAME).ok_or("complete Arabic setup is missing")?;
    if setup.url != update.installer.url
        || setup.size != update.installer.size
        || !setup.sha256.as_deref().is_some_and(|hash| hash.eq_ignore_ascii_case(&update.installer.sha256))
    {
        return Err("published setup and Arabic update manifest differ".into());
    }
    let status = if update.newer_than(current) { SuiteStatus::Available(Box::new(update)) } else { SuiteStatus::Current };
    Ok(status)
}

pub fn download(http: &Http, paths: &Paths, update: &SuiteRelease, stop: &Stop, phase: &mut dyn FnMut(JobPhase)) -> Result<PathBuf, String> {
    update.validate(&update.metadata.tag).map_err(str::to_string)?;
    let file = paths.downloads().join(format!("arabic-suite-{}-{}", update.metadata.revision, suite::SETUP_NAME));
    http.download(&update.installer.url, &file, update.installer.size, &update.installer.sha256, stop, &mut |transfer| {
        phase(match transfer {
            Transfer::Bytes { done, total } => JobPhase::Downloading { done, total },
            Transfer::Reconnecting { done, total } => JobPhase::Reconnecting { done, total },
        });
    })?;
    phase(JobPhase::Verifying);
    Ok(file)
}

#[cfg(any(windows, test))]
fn ps_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

#[cfg(windows)]
pub fn start_installer(file: &Path, paths: &Paths, language: &str, update: &SuiteRelease) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let dir = exe.parent().ok_or("launcher install directory is unavailable")?;
    installed(paths)?;
    update.validate(&update.metadata.tag).map_err(str::to_string)?;
    let script =
        helper_script(&exe, file, dir, if language == "ar" { "ar" } else { "en" }, &update.metadata, std::process::id(), &update.installer.sha256)?;
    let helper = paths.data.join("suite-update-helper.ps1");
    let mut bytes = b"\xef\xbb\xbf".to_vec();
    bytes.extend_from_slice(script.as_bytes());
    write_atomic(&helper, &bytes)?;
    let mut command = crate::process::command("powershell.exe");
    command.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"]).arg(&helper);
    crate::process::spawn_detached(command)
}

#[cfg(not(windows))]
pub fn start_installer(_file: &Path, _paths: &Paths, _language: &str, _update: &SuiteRelease) -> Result<(), String> {
    Err("the integrated Arabic setup supports Windows x64".into())
}

#[cfg(any(windows, test))]
fn helper_script(
    exe: &Path,
    installer: &Path,
    directory: &Path,
    language: &str,
    metadata: &Metadata,
    pid: u32,
    checksum: &str,
) -> Result<String, String> {
    metadata.validate().map_err(str::to_string)?;
    let expected = serde_json::to_string(metadata).map_err(|e| e.to_string())?;
    Ok(format!(
        r#"$ErrorActionPreference='Stop'
$launcher={exe}
$installer={installer}
$directory={directory}
$expected={expected} | ConvertFrom-Json
function Get-SuiteHash([string]$Path) {{
    $stream=$null;$algorithm=$null
    try {{
        $stream=[IO.File]::OpenRead($Path)
        $algorithm=[Security.Cryptography.SHA256]::Create()
        return [BitConverter]::ToString($algorithm.ComputeHash($stream)).Replace('-','').ToLowerInvariant()
    }} finally {{
        if($stream) {{$stream.Dispose()}}
        if($algorithm) {{$algorithm.Dispose()}}
    }}
}}
$resultPath=Join-Path $directory 'ArtCraftLauncherData\data\suite-update-result.json'
$result=@{{success=$false;requested_revision={revision};exit_code=$null;reason='waiting'}}
try {{
    $old=Get-Process -Id {pid} -ErrorAction SilentlyContinue
    if($old) {{
        if($old.Path -ne $launcher) {{throw 'Launcher identity changed'}}
        try {{ Wait-Process -Id {pid} -Timeout 180 -ErrorAction Stop }} catch {{
            if(Get-Process -Id {pid} -ErrorAction SilentlyContinue) {{throw}}
        }}
    }}
    if((Get-SuiteHash $installer) -ne {checksum}) {{throw 'Setup checksum changed'}}
    $arguments=@('/NORESTART','/SUITEUPDATE=1','/LANG={language}',('/DIR="'+$directory+'"'))
    $setup=Start-Process -FilePath $installer -ArgumentList $arguments -Wait -PassThru
    $result.exit_code=$setup.ExitCode
    if($setup.ExitCode -ne 0) {{throw 'Setup was cancelled or failed'}}
    $metadata=Get-Content -LiteralPath (Join-Path $directory 'ArtCraftLauncherData\data\bundle.json') -Raw | ConvertFrom-Json
    if($metadata.revision -ne {revision}) {{throw 'Setup did not install the requested Arabic revision'}}
    foreach($field in @('schema','edition','platform','install_id','tag','revision')) {{
        if($metadata.$field -ne $expected.$field) {{throw 'Installed metadata differs from the reviewed update'}}
    }}
    if(@($metadata.PSObject.Properties).Count -ne @($expected.PSObject.Properties).Count) {{throw 'Unexpected installed metadata fields'}}
    $rootPrefix=[IO.Path]::GetFullPath($directory).TrimEnd('\')+'\'
    $records=Get-Content -LiteralPath (Join-Path $directory 'ArtCraftLauncherData\data\installed.json') -Raw | ConvertFrom-Json
    $components=@($expected.applications.PSObject.Properties)
    if($components.Count -lt {minimum_apps} -or $components.Count -gt {maximum_apps} -or @($metadata.applications.PSObject.Properties).Count -ne $components.Count) {{throw 'Incomplete Arabic suite metadata'}}
    foreach($component in $components) {{
        $installedComponent=$metadata.applications.PSObject.Properties[$component.Name].Value
        if(!$installedComponent -or $installedComponent.version -ne $component.Value.version -or $installedComponent.sha256 -ne $component.Value.sha256) {{throw 'Installed component metadata differs from the reviewed update'}}
        $record=$records.apps.PSObject.Properties[$component.Name].Value
        if(!$record -or $record.version -ne $component.Value.version) {{throw 'Application record differs from Arabic metadata'}}
        $binary=[IO.Path]::GetFullPath((Join-Path (Join-Path $directory 'ArtCraftLauncherData\data') $record.path))
        if(!$binary.StartsWith($rootPrefix,[StringComparison]::OrdinalIgnoreCase) -or [IO.Path]::GetFileName($binary) -ne ($component.Name+'.exe')) {{throw 'Application path is outside this installation'}}
        if((Get-SuiteHash $binary) -ne $component.Value.sha256) {{throw 'Application checksum differs from reviewed Arabic metadata'}}
    }}
    if($metadata.launcher.version -ne $expected.launcher.version -or $metadata.launcher.sha256 -ne $expected.launcher.sha256) {{throw 'Launcher metadata differs from the reviewed update'}}
    if((Get-SuiteHash (Join-Path $directory 'artcraft-launcher.exe')) -ne $expected.launcher.sha256) {{throw 'Launcher checksum differs from reviewed Arabic metadata'}}
    $result.success=$true;$result.reason='installed' 
}} catch {{ $result.reason='cancelled_or_failed' }}
try {{[IO.File]::WriteAllText($resultPath,($result|ConvertTo-Json -Compress),[Text.UTF8Encoding]::new($false))}} catch {{}}
if(Test-Path -LiteralPath $launcher) {{Start-Process -FilePath $launcher -WorkingDirectory $directory -WindowStyle Hidden}}
"#,
        checksum = ps_literal(checksum),
        exe = ps_literal(&exe.to_string_lossy()),
        installer = ps_literal(&installer.to_string_lossy()),
        directory = ps_literal(&directory.to_string_lossy()),
        expected = ps_literal(&expected),
        revision = metadata.revision,
        minimum_apps = artcraft_launcher_core::SUITE.len(),
        maximum_apps = suite::MAX_APPLICATIONS
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (SuiteRelease, String, serde_json::Value) {
        let tag = "v2026.10-ar.2";
        let release = SuiteRelease {
            metadata: Metadata {
                schema: 1,
                edition: suite::EDITION.into(),
                platform: suite::PLATFORM.into(),
                install_id: suite::INSTALL_ID.into(),
                revision: 2,
                tag: tag.into(),
                launcher: suite::Component { version: "0.4.1".into(), sha256: "c".repeat(64) },
                applications: artcraft_launcher_core::SUITE
                    .iter()
                    .map(|app| (app.id.into(), suite::Component { version: "0.6.1".into(), sha256: "a".repeat(64) }))
                    .collect(),
            },
            installer: suite::Installer {
                name: suite::SETUP_NAME.into(),
                url: suite::asset_url(tag, suite::SETUP_NAME),
                size: 1000,
                sha256: "b".repeat(64),
            },
        };
        let body = serde_json::to_string(&release).unwrap();
        let json = serde_json::json!({"tag_name":tag,"draft":false,"prerelease":false,"html_url":suite::release_url(tag),"assets":[
            {"name":suite::FEED_NAME,"browser_download_url":suite::asset_url(tag,suite::FEED_NAME),"size":body.len(),"digest":format!("sha256:{:x}",Sha256::digest(body.as_bytes()))},
            {"name":suite::SETUP_NAME,"browser_download_url":release.installer.url,"size":1000,"digest":format!("sha256:{}",release.installer.sha256)}
        ]});
        (release, body, json)
    }
    #[test]
    fn published_feed_requires_complete_verified_own_release_and_a_new_revision() {
        let (release, body, json) = fixture();
        let published = published_release(&json.to_string()).unwrap();
        let mut installed = release.metadata.clone();
        installed.revision = 1;
        assert!(matches!(evaluate_feed(&published, &body, &installed).unwrap(), SuiteStatus::Available(_)));
        installed.revision = 2;
        assert_eq!(evaluate_feed(&published, &body, &installed).unwrap(), SuiteStatus::Current);
        installed.revision = 3;
        assert_eq!(evaluate_feed(&published, &body, &installed).unwrap(), SuiteStatus::Current);
        for fault in 0..8 {
            let mut json = json.clone();
            let mut body = body.clone();
            match fault {
                0 => json["draft"] = true.into(),
                1 => json["prerelease"] = true.into(),
                2 => json["assets"][0]["browser_download_url"] = "https://github.com/foreign/repo/releases/download/x/feed.json".into(),
                3 => json["assets"][0]["digest"] = serde_json::Value::Null,
                4 => json["assets"][1]["digest"] = format!("sha256:{}", "c".repeat(64)).into(),
                5 => json["assets"][1]["size"] = 999.into(),
                6 => body.push(' '),
                _ => json["tag_name"] = "v2026.13-ar.2".into(),
            }
            let answer = published_release(&json.to_string()).and_then(|published| evaluate_feed(&published, &body, &installed));
            assert!(answer.is_err(), "fault {fault}");
        }
    }
    #[test]
    fn persisted_failure_never_becomes_success_and_settings_remain_unchanged() {
        let directory = tempfile::tempdir().unwrap();
        let paths = Paths::under(directory.path());
        let (release, _, _) = fixture();
        std::fs::create_dir_all(&paths.data).unwrap();
        write_atomic(&paths.data.join("bundle.json"), &serde_json::to_vec(&release.metadata).unwrap()).unwrap();
        let prefs = directory.path().join("app.ron");
        std::fs::write(&prefs, b"user settings with Arabic preference").unwrap();
        save_status(&paths, &SuiteStatus::Failed).unwrap();
        record_attempt(&paths, 100000).unwrap();
        let restarted = Paths::under(directory.path());
        assert_eq!(saved_status(&restarted), Some(SuiteStatus::Failed));
        assert!(!due(last_attempt(&restarted), 100001, false));
        for (success, exit, revision, expected) in [(false, 0, 2, false), (true, 1, 2, false), (true, 0, 3, false), (true, 0, 2, true)] {
            let result = serde_json::json!({"success":success,"exit_code":exit,"requested_revision":revision});
            write_atomic(&paths.data.join("suite-update-result.json"), &serde_json::to_vec(&result).unwrap()).unwrap();
            assert_eq!(completed_install(&restarted), Some(expected));
            assert_eq!(completed_install(&restarted), None);
            assert_eq!(std::fs::read(&prefs).unwrap(), b"user settings with Arabic preference");
        }
    }
    #[test]
    fn daily_attempts_survive_offline_checks_and_force_refresh() {
        assert!(due(None, 1000, false));
        assert!(!due(Some(1000), 1001, false));
        assert!(!due(Some(1000), 1000 + suite::CHECK_INTERVAL - 1, false));
        assert!(due(Some(1000), 1000 + suite::CHECK_INTERVAL, false));
        assert!(due(Some(1000), 1001, true));
        assert!(due(Some(2000), 1000, false));
    }
    #[test]
    fn helper_preserves_arabic_apostrophes_and_requires_successful_installation() {
        let script = helper_script(
            Path::new("C:/O'Brien/العربية/launcher.exe"),
            Path::new("C:/cache/setup.exe"),
            Path::new("C:/O'Brien/العربية"),
            "ar",
            &fixture().0.metadata,
            123,
            &"a".repeat(64),
        )
        .unwrap();
        assert!(script.contains("O''Brien/العربية"));
        assert!(script.contains("-Timeout 180 -ErrorAction Stop"));
        assert!(script.contains("$setup.ExitCode -ne 0"));
        assert!(script.contains("$metadata.revision -ne 2"));
        assert!(!script.contains("/SILENT"));
        assert!(!script.contains("/VERYSILENT"));
    }

    /// Exercise the generated helper in Windows PowerShell, with only process launches mocked.
    /// Real JSON, SHA-256, path handling and result persistence run in an isolated installation.
    #[cfg(windows)]
    #[test]
    fn windows_helper_checks_reviewed_files_preserves_settings_and_restarts_after_failure() {
        for case in [
            "complete",
            "cancelled",
            "setup_changed",
            "version_changed",
            "binary_and_manifest_changed",
            "tag_changed",
            "future_application",
            "extra_application",
            "path_escape",
        ] {
            let directory = tempfile::Builder::new().prefix("suite O'Brien العربية ").tempdir().unwrap();
            let root = directory.path();
            let data = root.join("ArtCraftLauncherData/data");
            std::fs::create_dir_all(&data).unwrap();
            let launcher = root.join("artcraft-launcher.exe");
            let installer = root.join("setup.exe");
            std::fs::write(&launcher, b"test launcher; never executed").unwrap();
            std::fs::write(&installer, b"test installer; never executed").unwrap();
            let digest = |path: &Path| format!("{:x}", Sha256::digest(std::fs::read(path).unwrap()));
            let mut expected = fixture().0.metadata;
            expected.launcher.sha256 = digest(&launcher);
            if case == "future_application" {
                expected.applications.insert("futurecraft".into(), suite::Component { version: "0.1.0".into(), sha256: "a".repeat(64) });
            }
            let mut records = serde_json::Map::new();
            for (name, component) in &mut expected.applications {
                let binary = root.join(format!("apps/{name}/{name}.exe"));
                std::fs::create_dir_all(binary.parent().unwrap()).unwrap();
                std::fs::write(&binary, format!("isolated application {name}")).unwrap();
                component.sha256 = digest(&binary);
                records.insert(name.clone(), serde_json::json!({"version":component.version,"path":format!("../../apps/{name}/{name}.exe")}));
            }
            let mut installed = serde_json::to_value(&expected).unwrap();
            match case {
                "setup_changed" => std::fs::write(&installer, b"changed installer").unwrap(),
                "version_changed" => {
                    installed["applications"]["pdfcraft"]["version"] = "0.0.0".into();
                    records.get_mut("pdfcraft").unwrap()["version"] = "0.0.0".into();
                }
                "binary_and_manifest_changed" => {
                    let binary = root.join("apps/pdfcraft/pdfcraft.exe");
                    std::fs::write(&binary, b"replaced binary").unwrap();
                    installed["applications"]["pdfcraft"]["sha256"] = digest(&binary).into();
                }
                "tag_changed" => installed["tag"] = "v2026.11-ar.2".into(),
                "extra_application" => installed["applications"]["unreviewedcraft"] = serde_json::json!({"version":"0.1.0","sha256":"a".repeat(64)}),
                "path_escape" => records.get_mut("pdfcraft").unwrap()["path"] = "../../../pdfcraft.exe".into(),
                _ => (),
            }
            std::fs::write(data.join("bundle.json"), serde_json::to_vec(&installed).unwrap()).unwrap();
            std::fs::write(data.join("installed.json"), serde_json::to_vec(&serde_json::json!({"apps":records})).unwrap()).unwrap();
            let preferences = root.join("app.ron");
            let original_preferences = "(language:Ar, user_setting:\"إعداد محفوظ\")";
            std::fs::write(&preferences, original_preferences).unwrap();
            let restart = root.join("restart.txt");
            let checksum = format!("{:x}", Sha256::digest(b"test installer; never executed"));
            let helper = helper_script(&launcher, &installer, root, "ar", &expected, 2_147_483_646, &checksum)
                .unwrap()
                .replace("$result.reason='cancelled_or_failed'", "$result.reason=$_.Exception.Message");
            let mock = format!(
                r#"$testInstaller={installer}
$testLauncher={launcher}
$testRestart={restart}
$testExit={exit}
function Start-Process {{
    param([string]$FilePath,[string[]]$ArgumentList,[switch]$Wait,[switch]$PassThru,[string]$WorkingDirectory,[string]$WindowStyle)
    if($FilePath -eq $testInstaller) {{
        if(!$Wait -or !$PassThru -or $ArgumentList -notcontains '/SUITEUPDATE=1' -or $ArgumentList -notcontains '/LANG=ar' -or $ArgumentList -notcontains ('/DIR="'+[IO.Path]::GetDirectoryName($testLauncher)+'"')) {{throw 'Invalid installer arguments'}}
        return [pscustomobject]@{{ExitCode=$testExit}}
    }}
    if($FilePath -eq $testLauncher) {{
        if(!(Test-Path -LiteralPath (Join-Path ([IO.Path]::GetDirectoryName($testLauncher)) 'ArtCraftLauncherData\data\suite-update-result.json'))) {{throw 'Launcher restarted before update result was written'}}
        [IO.File]::WriteAllText($testRestart,'restarted');return
    }}
    throw 'Unexpected process launch in test'
}}
"#,
                installer = ps_literal(&installer.to_string_lossy()),
                launcher = ps_literal(&launcher.to_string_lossy()),
                restart = ps_literal(&restart.to_string_lossy()),
                exit = if case == "cancelled" { 1602 } else { 0 }
            );
            let script_path = root.join("helper.ps1");
            let mut bytes = b"\xef\xbb\xbf".to_vec();
            bytes.extend_from_slice(mock.as_bytes());
            bytes.extend_from_slice(helper.as_bytes());
            std::fs::write(&script_path, bytes).unwrap();
            let output = crate::process::command("powershell.exe")
                .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"])
                .arg(script_path)
                .output()
                .unwrap();
            assert!(output.status.success(), "{case}: {}", String::from_utf8_lossy(&output.stderr));
            let result: serde_json::Value = serde_json::from_slice(&std::fs::read(data.join("suite-update-result.json")).unwrap()).unwrap();
            assert_eq!(result["success"], matches!(case, "complete" | "future_application"), "{case}: {result}");
            assert_eq!(result["requested_revision"], 2);
            assert_eq!(std::fs::read_to_string(&preferences).unwrap(), original_preferences, "{case}");
            assert!(restart.is_file(), "launcher must restart after {case}");
        }
    }
}
