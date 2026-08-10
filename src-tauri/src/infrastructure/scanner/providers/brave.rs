use crate::domain::types::{BrowserChannel, BrowserFamily};
use crate::infrastructure::scanner::models::BrowserScanResult;
use crate::infrastructure::scanner::provider::{scan_chromium_profiles, BrowserProvider};
use std::path::PathBuf;

pub struct BraveProvider;

impl BrowserProvider for BraveProvider {
    fn browser_family(&self) -> BrowserFamily {
        BrowserFamily::Brave
    }

    fn browser_channel(&self) -> BrowserChannel {
        BrowserChannel::Stable
    }

    fn scan(&self) -> BrowserScanResult {
        let mut extensions = Vec::new();

        #[cfg(target_os = "windows")]
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let path =
                PathBuf::from(local_app_data).join("BraveSoftware\\Brave-Browser\\User Data");
            extensions = scan_chromium_profiles(&path);
        }

        BrowserScanResult {
            browser_family: self.browser_family(),
            browser_channel: self.browser_channel(),
            extensions,
            error: None,
        }
    }
}
