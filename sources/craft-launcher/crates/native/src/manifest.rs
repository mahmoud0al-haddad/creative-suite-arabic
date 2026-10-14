//! The launcher's record of what it installed itself.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use artcraft_launcher_core::PackageKind;
use serde::{Deserialize, Serialize};

/// One app the launcher installed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub version: String,
    pub kind: PackageKind,
    /// The `.app`, the exe, the AppImage, or the folder of an unpacked tarball.
    pub path: PathBuf,
    /// Unix seconds.
    pub installed_at: i64,
    /// Every file installed, for tree installs (FreeBSD), so removal takes exactly those.
    #[serde(default)]
    pub files: Vec<PathBuf>,
    /// The files are in a system prefix and removing them needs root.
    #[serde(default)]
    pub needs_root: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub apps: BTreeMap<String, Record>,
}

impl Manifest {
    pub fn load(path: &Path) -> Self {
        let mut manifest: Self = std::fs::read(path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        if let Some(folder) = path.parent() {
            for record in manifest.apps.values_mut() {
                if !record.path.has_root() {
                    record.path = folder.join(&record.path);
                }
            }
        }
        manifest
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let mut saved = self.clone();
        if let Some(folder) = path.parent() {
            for record in saved.apps.values_mut() {
                if let Ok(relative) = record.path.strip_prefix(folder) {
                    record.path = relative.to_owned();
                }
            }
        }
        let bytes = serde_json::to_vec_pretty(&saved).map_err(|e| e.to_string())?;
        crate::paths::write_atomic(path, &bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_bundle_paths_follow_the_manifest_after_relocation() {
        for _ in 0..2 {
            let root = tempfile::tempdir().unwrap();
            let path = root.path().join("installed.json");
            let relative = PathBuf::from("apps/photocraft/photocraft.exe");
            let mut manifest = Manifest::default();
            manifest.apps.insert(
                "photocraft".into(),
                Record {
                    version: "0.7.0".into(),
                    kind: PackageKind::PortableZip,
                    path: relative.clone(),
                    installed_at: 0,
                    files: vec![],
                    needs_root: false,
                },
            );
            manifest.save(&path).unwrap();
            let loaded = Manifest::load(&path);
            assert_eq!(loaded.apps["photocraft"].path, root.path().join(&relative));
            loaded.save(&path).unwrap();
            let saved: Manifest = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            assert_eq!(saved.apps["photocraft"].path, relative);
        }
    }

    #[test]
    fn round_trips_and_survives_garbage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("installed.json");
        assert_eq!(Manifest::load(&path), Manifest::default());
        let mut m = Manifest::default();
        m.apps.insert(
            "photocraft".into(),
            Record { version: "0.5.0".into(), kind: PackageKind::AppImage, path: "/x/y".into(), installed_at: 1, files: vec![], needs_root: false },
        );
        m.save(&path).unwrap();
        assert_eq!(Manifest::load(&path), m);
        std::fs::write(&path, b"{not json").unwrap();
        assert_eq!(Manifest::load(&path), Manifest::default());
    }
}
