use super::models::{BrowserScanResult, RawDiscoveredExtension};
use crate::domain::types::{BrowserChannel, BrowserFamily};
use std::fs;
use std::path::Path;

pub trait BrowserProvider: Send + Sync {
    fn browser_family(&self) -> BrowserFamily;
    fn browser_channel(&self) -> BrowserChannel;
    fn scan(&self) -> BrowserScanResult;
}

use serde_json::Value;
use std::collections::HashMap;

pub fn read_extension_states(profile_dir: &Path) -> HashMap<String, bool> {
    let mut ext_states = HashMap::new();
    let prefs_path = profile_dir.join("Secure Preferences");
    let prefs_path = if prefs_path.exists() {
        prefs_path
    } else {
        profile_dir.join("Preferences")
    };

    if let Ok(content) = fs::read_to_string(&prefs_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(settings) = json.pointer("/extensions/settings") {
                if let Some(settings_obj) = settings.as_object() {
                    for (ext_id, ext_data) in settings_obj {
                        if let Some(state) = ext_data.get("state").and_then(|v| v.as_i64()) {
                            ext_states.insert(ext_id.clone(), state == 1);
                        }
                    }
                }
            }
        }
    }
    ext_states
}

/// Helper function to scan a standard Chromium profile structure
pub fn scan_chromium_profiles(user_data_path: &Path) -> Vec<RawDiscoveredExtension> {
    let mut extensions = Vec::new();

    if !user_data_path.exists() || !user_data_path.is_dir() {
        return extensions;
    }

    // Iterate through profiles (Default, Profile 1, etc.)
    if let Ok(entries) = fs::read_dir(user_data_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let profile_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let ext_states = read_extension_states(&path);

                // Exclude obvious non-profile dirs if we want, but checking for "Extensions" folder is safe enough
                let extensions_dir = path.join("Extensions");
                if extensions_dir.exists() && extensions_dir.is_dir() {
                    extensions.extend(scan_extensions_directory(
                        &extensions_dir,
                        &profile_name,
                        &ext_states,
                    ));
                }
            }
        }
    }

    extensions
}

/// Helper function to scan the extensions directory for standard Chromium structures
pub fn scan_extensions_directory(
    extensions_dir: &Path,
    profile_name: &str,
    ext_states: &HashMap<String, bool>,
) -> Vec<RawDiscoveredExtension> {
    let mut extensions = Vec::new();

    if let Ok(entries) = fs::read_dir(extensions_dir) {
        for entry in entries.flatten() {
            let ext_path = entry.path();
            if ext_path.is_dir() {
                let ext_id = ext_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                // Simple validation: Chromium extension IDs are 32 chars [a-p]
                if ext_id.len() == 32
                    && ext_id
                        .chars()
                        .all(|c| c.is_ascii_lowercase() && ('a'..='p').contains(&c))
                {
                    let is_enabled = ext_states.get(&ext_id).copied().unwrap_or(false);
                    let disabled = !is_enabled;

                    // Iterate through versions
                    if let Ok(versions) = fs::read_dir(&ext_path) {
                        for version_entry in versions.flatten() {
                            let version_path = version_entry.path();
                            if version_path.is_dir() {
                                let manifest_path = version_path.join("manifest.json");
                                if manifest_path.exists() && manifest_path.is_file() {
                                    let version_str = version_path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string();

                                    let mut name = "Unknown".to_string();
                                    let mut version = version_str.clone();
                                    let mut manifest_version = 2;

                                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                            if let Some(n) =
                                                json.get("name").and_then(|v| v.as_str())
                                            {
                                                name = n.to_string();
                                            }
                                            if let Some(v) =
                                                json.get("version").and_then(|v| v.as_str())
                                            {
                                                version = v.to_string();
                                            }
                                            if let Some(mv) = json
                                                .get("manifest_version")
                                                .and_then(|v| v.as_i64())
                                            {
                                                manifest_version = mv as i32;
                                            }
                                        }
                                    }

                                    extensions.push(RawDiscoveredExtension {
                                        extension_id: ext_id.clone(),
                                        profile_name: profile_name.to_string(),
                                        version,
                                        install_path: version_path.to_string_lossy().to_string(),
                                        name,
                                        manifest_version,
                                        disabled,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    extensions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_extension_states() {
        let dir = tempdir().unwrap();
        let prefs_path = dir.path().join("Preferences");
        let mut file = File::create(prefs_path).unwrap();
        file.write_all(
            b"{\"extensions\": {\"settings\": {\"abc\": {\"state\": 1}, \"def\": {\"state\": 0}}}}",
        )
        .unwrap();

        let states = read_extension_states(dir.path());
        assert_eq!(states.get("abc"), Some(&true));
        assert_eq!(states.get("def"), Some(&false));
        assert_eq!(states.get("ghi"), None);
    }

    #[test]
    fn test_scan_extensions_directory() {
        let dir = tempdir().unwrap();
        // Fake extension ID 32 chars
        let ext_id = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let ext_dir = dir.path().join(ext_id);
        fs::create_dir(&ext_dir).unwrap();

        let version_dir = ext_dir.join("1.0.0_0");
        fs::create_dir(&version_dir).unwrap();

        let manifest_path = version_dir.join("manifest.json");
        let mut file = File::create(manifest_path).unwrap();
        file.write_all(
            b"{\"name\": \"Test Ext\", \"version\": \"1.0.0\", \"manifest_version\": 3}",
        )
        .unwrap();

        let mut states = HashMap::new();
        states.insert(ext_id.to_string(), true);

        let results = scan_extensions_directory(dir.path(), "Default", &states);
        assert_eq!(results.len(), 1);
        let ext = &results[0];
        assert_eq!(ext.extension_id, ext_id);
        assert_eq!(ext.name, "Test Ext");
        assert_eq!(ext.version, "1.0.0");
        assert_eq!(ext.manifest_version, 3);
        assert_eq!(ext.disabled, false);
    }
}
