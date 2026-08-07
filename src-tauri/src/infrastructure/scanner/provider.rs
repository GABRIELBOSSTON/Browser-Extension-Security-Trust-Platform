use crate::domain::types::{BrowserFamily, BrowserChannel};
use super::models::{BrowserScanResult, RawDiscoveredExtension};
use std::path::Path;
use std::fs;

pub trait BrowserProvider: Send + Sync {
    fn browser_family(&self) -> BrowserFamily;
    fn browser_channel(&self) -> BrowserChannel;
    fn scan(&self) -> BrowserScanResult;
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
                let profile_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // Exclude obvious non-profile dirs if we want, but checking for "Extensions" folder is safe enough
                let extensions_dir = path.join("Extensions");
                if extensions_dir.exists() && extensions_dir.is_dir() {
                    extensions.extend(scan_extensions_directory(&extensions_dir, &profile_name));
                }
            }
        }
    }

    extensions
}

/// Helper function to scan the extensions directory for standard Chromium structures
pub fn scan_extensions_directory(extensions_dir: &Path, profile_name: &str) -> Vec<RawDiscoveredExtension> {
    let mut extensions = Vec::new();

    if let Ok(entries) = fs::read_dir(extensions_dir) {
        for entry in entries.flatten() {
            let ext_path = entry.path();
            if ext_path.is_dir() {
                let ext_id = ext_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // Simple validation: Chromium extension IDs are 32 chars [a-p]
                if ext_id.len() == 32 && ext_id.chars().all(|c| c.is_ascii_lowercase() && c >= 'a' && c <= 'p') {
                    // Iterate through versions
                    if let Ok(versions) = fs::read_dir(&ext_path) {
                        for version_entry in versions.flatten() {
                            let version_path = version_entry.path();
                            if version_path.is_dir() {
                                let manifest_path = version_path.join("manifest.json");
                                if manifest_path.exists() && manifest_path.is_file() {
                                    let version_str = version_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    
                                    extensions.push(RawDiscoveredExtension {
                                        extension_id: ext_id.clone(),
                                        profile_name: profile_name.to_string(),
                                        version: version_str,
                                        install_path: version_path.to_string_lossy().to_string(),
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
