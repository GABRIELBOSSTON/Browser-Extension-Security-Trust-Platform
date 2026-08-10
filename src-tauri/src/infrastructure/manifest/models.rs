use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RawManifest {
    pub manifest_version: Option<u32>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub permissions: Vec<serde_json::Value>,
    #[serde(default)]
    pub host_permissions: Vec<serde_json::Value>,
    #[serde(default)]
    pub optional_permissions: Vec<serde_json::Value>,
    pub background: Option<serde_json::Value>,
    pub action: Option<serde_json::Value>,
    pub browser_action: Option<serde_json::Value>,
    pub page_action: Option<serde_json::Value>,
    pub icons: Option<serde_json::Value>,
    #[serde(default)]
    pub content_scripts: Vec<serde_json::Value>,
    #[serde(default)]
    pub web_accessible_resources: Vec<serde_json::Value>,
    pub content_security_policy: Option<serde_json::Value>,
    pub externally_connectable: Option<serde_json::Value>,
}
