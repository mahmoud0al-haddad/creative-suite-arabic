//! The independent edition's complete bundle update contract. Component updates cannot replace Arabic builds.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const REPOSITORY: &str = "mahmoud0al-haddad/creative-suite-arabic";
pub const EDITION: &str = "creative-suite-arabic-community";
pub const PLATFORM: &str = "windows-x64";
pub const INSTALL_ID: &str = "149F3153-387D-4B57-9478-74333EAA5D28";
pub const SETUP_NAME: &str = "Creative-Suite-Arabic-Windows-x64-Setup.exe";
pub const FEED_NAME: &str = "arabic-suite-release.json";
pub const CHECK_INTERVAL: i64 = 24 * 60 * 60;
pub const MAX_SETUP_SIZE: u64 = 2 * 1024 * 1024 * 1024;
pub const MAX_APPLICATIONS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    pub version: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub schema: u32,
    pub edition: String,
    pub platform: String,
    pub install_id: String,
    pub revision: u64,
    pub tag: String,
    pub launcher: Component,
    pub applications: BTreeMap<String, Component>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Installer {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SuiteRelease {
    pub metadata: Metadata,
    pub installer: Installer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuiteStatus {
    Current,
    NotPublished,
    Available(Box<SuiteRelease>),
    Failed,
}

pub fn valid_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

pub fn valid_application_id(value: &str) -> bool {
    (1..=64).contains(&value.len())
        && value.bytes().next().is_some_and(|byte| byte.is_ascii_lowercase())
        && value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

pub fn valid_tag(tag: &str) -> bool {
    let Some(tag) = tag.strip_prefix('v') else { return false };
    let Some((calendar, revision)) = tag.split_once("-ar.") else { return false };
    let Some((year, month)) = calendar.split_once('.') else { return false };
    if [year, month, revision].iter().any(|v| v.is_empty() || !v.bytes().all(|b| b.is_ascii_digit())) {
        return false;
    }
    matches!(year.parse::<u32>(), Ok(2020..=9999)) && matches!(month.parse::<u32>(), Ok(1..=12)) && matches!(revision.parse::<u32>(), Ok(1..))
}

pub fn release_url(tag: &str) -> String {
    format!("https://github.com/{REPOSITORY}/releases/tag/{tag}")
}

pub fn asset_url(tag: &str, name: &str) -> String {
    format!("https://github.com/{REPOSITORY}/releases/download/{tag}/{name}")
}

impl Metadata {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema != 1 || self.edition != EDITION || self.platform != PLATFORM || self.install_id != INSTALL_ID {
            return Err("unsupported Arabic edition metadata");
        }
        // Revision zero describes a local pre-release installation; it is never offered by a release feed.
        if !valid_tag(&self.tag) {
            return Err("invalid Arabic edition revision");
        }
        if !(crate::SUITE.len()..=MAX_APPLICATIONS).contains(&self.applications.len())
            || crate::SUITE.iter().any(|app| !self.applications.contains_key(app.id))
        {
            return Err("the update must contain every bundled application");
        }
        if self.applications.keys().any(|id| !valid_application_id(id)) {
            return Err("invalid bundled application identifier");
        }
        if self.applications.values().any(|app| !valid_hash(&app.sha256) || crate::version::parse(&app.version).is_none()) {
            return Err("invalid application version or checksum");
        }
        if !valid_hash(&self.launcher.sha256) || crate::version::parse(&self.launcher.version).is_none() {
            return Err("invalid launcher version or checksum");
        }
        Ok(())
    }
}

impl SuiteRelease {
    pub fn validate(&self, published_tag: &str) -> Result<(), &'static str> {
        self.metadata.validate()?;
        if self.metadata.revision == 0 {
            return Err("a published Arabic edition needs a positive revision");
        }
        if self.metadata.tag != published_tag || self.installer.name != SETUP_NAME || self.installer.url != asset_url(published_tag, SETUP_NAME) {
            return Err("update does not belong to the published Arabic edition");
        }
        if self.installer.size == 0 || self.installer.size > MAX_SETUP_SIZE || !valid_hash(&self.installer.sha256) {
            return Err("invalid installer size or mandatory checksum");
        }
        Ok(())
    }

    pub fn newer_than(&self, installed: &Metadata) -> bool {
        self.metadata.revision > installed.revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn release() -> SuiteRelease {
        let tag = "v2026.10-ar.2".to_string();
        SuiteRelease {
            metadata: Metadata {
                schema: 1,
                edition: EDITION.into(),
                platform: PLATFORM.into(),
                install_id: INSTALL_ID.into(),
                revision: 2,
                tag: tag.clone(),
                launcher: Component { version: "0.4.1".into(), sha256: "c".repeat(64) },
                applications: crate::SUITE
                    .iter()
                    .map(|app| (app.id.to_string(), Component { version: "0.6.1".into(), sha256: "a".repeat(64) }))
                    .collect(),
            },
            installer: Installer { name: SETUP_NAME.into(), url: asset_url(&tag, SETUP_NAME), size: 1000, sha256: "b".repeat(64) },
        }
    }
    #[test]
    fn rejects_foreign_partial_unverified_and_unsupported_updates() {
        let good = release();
        assert!(good.validate(&good.metadata.tag).is_ok());
        for fault in 0..8 {
            let mut bad = good.clone();
            match fault {
                0 => bad.installer.url = "https://github.com/storytold/photocraft/releases/download/v0.8.2/app.exe".into(),
                1 => bad.installer.sha256.clear(),
                2 => bad.metadata.applications.clear(),
                3 => bad.metadata.schema = 2,
                4 => bad.installer.size = MAX_SETUP_SIZE + 1,
                5 => bad.metadata.tag = "../../file".into(),
                6 => bad.metadata.install_id.clear(),
                _ => bad
                    .metadata
                    .applications
                    .insert("../unknown-app".into(), Component { version: "0.1.0".into(), sha256: "a".repeat(64) })
                    .map_or((), |_| ()),
            }
            assert!(bad.validate(&good.metadata.tag).is_err(), "fault {fault}");
        }
    }
    #[test]
    fn a_complete_update_can_add_new_craft_applications() {
        let mut update = release();
        update.metadata.applications.insert("futurecraft".into(), Component { version: "0.1.0".into(), sha256: "d".repeat(64) });
        assert!(update.validate(&update.metadata.tag).is_ok(), "a complete newer suite must be able to add applications");
        let mut current = release().metadata;
        current.revision = 1;
        assert!(update.newer_than(&current));
    }

    #[test]
    fn additional_applications_cannot_escape_paths_or_exceed_the_bound() {
        for id in ["", "../app", "C:\\app", "/app", "UpperCraft", "two apps", "app.exe", "برنامج"] {
            let mut update = release();
            update.metadata.applications.insert(id.into(), Component { version: "0.1.0".into(), sha256: "d".repeat(64) });
            assert!(update.validate(&update.metadata.tag).is_err(), "unsafe identifier {id:?}");
        }
        let mut update = release();
        for n in 0..MAX_APPLICATIONS {
            update.metadata.applications.insert(format!("futurecraft-{n}"), Component { version: "0.1.0".into(), sha256: "d".repeat(64) });
        }
        assert!(update.validate(&update.metadata.tag).is_err());
        let mut update = release();
        update.metadata.applications.remove(crate::SUITE[0].id);
        update.metadata.applications.insert("futurecraft".into(), Component { version: "0.1.0".into(), sha256: "d".repeat(64) });
        assert!(update.validate(&update.metadata.tag).is_err(), "a new application cannot silently replace a current one");
    }

    #[test]
    fn revisions_prevent_repeat_installation_and_downgrades() {
        let release = release();
        let mut installed = release.metadata.clone();
        assert!(!release.newer_than(&installed));
        installed.revision = 3;
        assert!(!release.newer_than(&installed));
        installed.revision = 1;
        assert!(release.newer_than(&installed));
        assert!(valid_tag("v2026.10-ar.1"));
        assert!(!valid_tag("v2026.10-ar.1/foreign"));
        assert!(!valid_tag("v2026.13-ar.1"));
    }
}
