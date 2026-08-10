use crate::domain::types::{BrowserChannel, BrowserFamily};
use crate::infrastructure::scanner::models::BrowserScanResult;
use crate::infrastructure::scanner::provider::{scan_extensions_directory, BrowserProvider};
use std::path::PathBuf;

pub struct OperaStableProvider;

impl BrowserProvider for OperaStableProvider {
    fn browser_family(&self) -> BrowserFamily {
        BrowserFamily::Opera
    }

    fn browser_channel(&self) -> BrowserChannel {
        BrowserChannel::Stable
    }

    fn scan(&self) -> BrowserScanResult {
        let mut extensions = Vec::new();

        #[cfg(target_os = "windows")]
        if let Ok(app_data) = std::env::var("APPDATA") {
            let path = PathBuf::from(app_data).join("Opera Software\\Opera Stable\\Extensions");
            if path.exists() && path.is_dir() {
                let profile_dir = path.parent().unwrap_or(&path);
                let ext_states =
                    crate::infrastructure::scanner::provider::read_extension_states(profile_dir);
                // Opera typically uses a single default profile and stores extensions directly
                extensions.extend(scan_extensions_directory(&path, "Default", &ext_states));
            }
        }

        BrowserScanResult {
            browser_family: self.browser_family(),
            browser_channel: self.browser_channel(),
            extensions,
            error: None,
        }
    }
}
