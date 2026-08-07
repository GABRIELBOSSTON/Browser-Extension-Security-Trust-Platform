use serde::{Deserialize, Serialize};
use super::types::PermissionType;

use std::collections::HashMap;

/// Manifest Version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestVersion {
    V2,
    V3,
}

/// Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMetadata {
    pub version: String,
    pub name: String,
    pub description: Option<String>,
}

/// Background Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundConfig {
    V2 {
        scripts: Vec<String>,
        page: Option<String>,
        persistent: Option<bool>,
    },
    V3 {
        service_worker: String,
        worker_type: Option<String>,
    },
}

/// Action Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub default_popup: Option<String>,
    pub default_title: Option<String>,
}

/// Icon Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconConfig {
    pub icons: HashMap<String, String>,
}

/// Content Script Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScriptConfig {
    pub matches: Vec<String>,
    pub js: Vec<String>,
    pub css: Vec<String>,
    pub run_at: Option<String>,
}

/// Web Accessible Resource Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAccessibleResourceConfig {
    pub resources: Vec<String>,
    pub matches: Vec<String>, // Empty for V2
    pub extension_ids: Vec<String>,
    pub use_dynamic_url: Option<bool>,
}

/// Permission Set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionSet {
    pub items: Vec<Permission>,
}

/// Manifest entity aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest_version: ManifestVersion,
    pub metadata: ManifestMetadata,
    pub permissions: PermissionSet,
    pub host_permissions: PermissionSet,
    pub optional_permissions: PermissionSet,
    pub background: Option<BackgroundConfig>,
    pub action: Option<ActionConfig>,
    pub icons: Option<IconConfig>,
    pub content_scripts: Vec<ContentScriptConfig>,
    pub web_accessible_resources: Vec<WebAccessibleResourceConfig>,
}

/// Permission entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub permission_id: String,
    pub permission_string: String,
    pub permission_type: PermissionType,
    pub weight: f64,
}

/// Extension aggregate root entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredExtension {
    pub extension_id: String,
    pub name: String,
    pub version: String,
    pub browser_family: super::types::BrowserFamily,
    pub browser_channel: super::types::BrowserChannel,
    pub install_path: String,
    pub profile_name: String,
    pub disabled: bool,
    pub policy_installed: bool,
    pub manifest: Option<Manifest>,
}


