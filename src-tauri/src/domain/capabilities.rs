use serde::{Deserialize, Serialize};

/// Stable Identifier for a Capability
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityId(pub String);

/// Value object representing a parsed Match Pattern
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchPattern {
    pub raw: String,
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
}

impl MatchPattern {
    pub fn parse(raw: &str) -> Self {
        // Simplified parser for now
        let mut scheme = None;
        let mut host = None;
        let mut path = None;

        if raw == "<all_urls>" {
            scheme = Some("*".to_string());
            host = Some("*".to_string());
            path = Some("/*".to_string());
        } else if let Some((s, rest)) = raw.split_once("://") {
            scheme = Some(s.to_string());
            if let Some((h, p)) = rest.split_once('/') {
                host = Some(h.to_string());
                path = Some(format!("/{}", p));
            } else {
                host = Some(rest.to_string());
                path = Some("/*".to_string());
            }
        }

        Self {
            raw: raw.to_string(),
            scheme,
            host,
            path,
        }
    }
}

/// Standard Extension API Permission Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCapability {
    pub id: CapabilityId,
    pub name: String,
    pub optional: bool,
}

/// Collection of API Permissions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionCapabilities {
    pub items: Vec<PermissionCapability>,
}

/// Collection of Host Permissions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HostCapabilities {
    pub patterns: Vec<MatchPattern>,
}

/// Collection of Content Scripts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentScriptCapabilities {
    pub scripts: Vec<crate::domain::entities::ContentScriptConfig>,
}

/// Collection of Web Accessible Resources
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAccessibleCapabilities {
    pub resources: Vec<crate::domain::entities::WebAccessibleResourceConfig>,
}

/// Background Capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundCapabilities {
    pub config: Option<crate::domain::entities::BackgroundConfig>,
}

/// Action Capabilities (UI interactions)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionCapabilities {
    pub config: Option<crate::domain::entities::ActionConfig>,
}

/// Content Security Policy (CSP) Capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CSPCapabilities {
    pub raw_policy: Option<String>,
}

/// Aggregate Root representing all capabilities of an extension
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtensionCapabilityModel {
    pub permissions: PermissionCapabilities,
    pub hosts: HostCapabilities,
    pub background: BackgroundCapabilities,
    pub csp: CSPCapabilities,
    pub actions: ActionCapabilities,
    pub content_scripts: ContentScriptCapabilities,
    pub web_accessible_resources: WebAccessibleCapabilities,
}
