use crate::domain::entities::{
    ActionConfig, BackgroundConfig, ContentScriptConfig, IconConfig, Manifest, ManifestMetadata,
    ManifestVersion, Permission, PermissionSet, WebAccessibleResourceConfig,
};
use crate::domain::types::PermissionType;
use crate::infrastructure::manifest::models::RawManifest;
use std::collections::HashMap;

pub struct ManifestMapper;

impl ManifestMapper {
    pub fn map(raw: RawManifest) -> crate::domain::errors::Result<Manifest> {
        let version = match raw.manifest_version.unwrap_or(2) {
            2 => ManifestVersion::V2,
            3 => ManifestVersion::V3,
            v => return Err(crate::domain::errors::DomainError::UnsupportedManifestVersion(v)),
        };

        let metadata = ManifestMetadata {
            version: raw.version.unwrap_or_default(),
            name: raw.name.unwrap_or_default(),
            description: raw.description,
        };

        let permissions = Self::map_permissions(raw.permissions);
        let host_permissions = Self::map_permissions(raw.host_permissions);
        let optional_permissions = Self::map_permissions(raw.optional_permissions);

        let background = Self::map_background(&raw.background, version);
        let action = Self::map_action(&raw.action, &raw.browser_action, &raw.page_action, version);
        let icons = Self::map_icons(&raw.icons);
        let content_scripts = Self::map_content_scripts(raw.content_scripts);
        let web_accessible_resources =
            Self::map_web_accessible_resources(raw.web_accessible_resources, version);

        let content_security_policy =
            Self::map_content_security_policy(&raw.content_security_policy, version);
        let externally_connectable = Self::map_externally_connectable(&raw.externally_connectable);

        Ok(Manifest {
            manifest_version: version,
            metadata,
            permissions,
            host_permissions,
            optional_permissions,
            background,
            action,
            icons,
            content_scripts,
            web_accessible_resources,
            content_security_policy,
            externally_connectable,
        })
    }

    fn map_permissions(raw: Vec<serde_json::Value>) -> PermissionSet {
        let mut items = Vec::new();
        for val in raw {
            if let Some(s) = val.as_str() {
                items.push(Permission {
                    permission_id: s.to_string(),
                    permission_string: s.to_string(),
                    permission_type: PermissionType::HostPattern, // Default placeholder
                    weight: 0.0,                                  // Default placeholder
                });
            } else if let Some(obj) = val.as_object() {
                // Sometimes permissions can be objects (like in MV3 dicts for some features)
                // Fallback to storing a generic string repr for later analysis
                let s = serde_json::to_string(obj).unwrap_or_default();
                items.push(Permission {
                    permission_id: s.clone(),
                    permission_string: s,
                    permission_type: PermissionType::HostPattern,
                    weight: 0.0,
                });
            }
        }
        PermissionSet { items }
    }

    fn map_background(
        raw: &Option<serde_json::Value>,
        version: ManifestVersion,
    ) -> Option<BackgroundConfig> {
        let obj = raw.as_ref()?.as_object()?;
        match version {
            ManifestVersion::V2 => {
                let mut scripts = Vec::new();
                if let Some(s) = obj.get("scripts").and_then(|v| v.as_array()) {
                    for v in s {
                        if let Some(st) = v.as_str() {
                            scripts.push(st.to_string());
                        }
                    }
                }
                let page = obj
                    .get("page")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let persistent = obj.get("persistent").and_then(|v| v.as_bool());
                Some(BackgroundConfig::V2 {
                    scripts,
                    page,
                    persistent,
                })
            }
            ManifestVersion::V3 => {
                let service_worker = obj
                    .get("service_worker")
                    .and_then(|v| v.as_str())?
                    .to_string();
                let worker_type = obj
                    .get("type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                Some(BackgroundConfig::V3 {
                    service_worker,
                    worker_type,
                })
            }
        }
    }

    fn map_action(
        action: &Option<serde_json::Value>,
        browser_action: &Option<serde_json::Value>,
        page_action: &Option<serde_json::Value>,
        version: ManifestVersion,
    ) -> Option<ActionConfig> {
        let obj = match version {
            ManifestVersion::V3 => action.as_ref()?.as_object()?,
            ManifestVersion::V2 => browser_action
                .as_ref()
                .or(page_action.as_ref())?
                .as_object()?,
        };
        let default_popup = obj
            .get("default_popup")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let default_title = obj
            .get("default_title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        Some(ActionConfig {
            default_popup,
            default_title,
        })
    }

    fn map_icons(raw: &Option<serde_json::Value>) -> Option<IconConfig> {
        let obj = raw.as_ref()?.as_object()?;
        let mut icons = HashMap::new();
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                icons.insert(k.clone(), s.to_string());
            }
        }
        Some(IconConfig { icons })
    }

    fn map_content_scripts(raw: Vec<serde_json::Value>) -> Vec<ContentScriptConfig> {
        let mut scripts = Vec::new();
        for val in raw {
            if let Some(obj) = val.as_object() {
                let mut matches = Vec::new();
                if let Some(m) = obj.get("matches").and_then(|v| v.as_array()) {
                    for v in m {
                        if let Some(s) = v.as_str() {
                            matches.push(s.to_string());
                        }
                    }
                }
                let mut js = Vec::new();
                if let Some(j) = obj.get("js").and_then(|v| v.as_array()) {
                    for v in j {
                        if let Some(s) = v.as_str() {
                            js.push(s.to_string());
                        }
                    }
                }
                let mut css = Vec::new();
                if let Some(c) = obj.get("css").and_then(|v| v.as_array()) {
                    for v in c {
                        if let Some(s) = v.as_str() {
                            css.push(s.to_string());
                        }
                    }
                }
                let run_at = obj
                    .get("run_at")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                scripts.push(ContentScriptConfig {
                    matches,
                    js,
                    css,
                    run_at,
                });
            }
        }
        scripts
    }

    fn map_web_accessible_resources(
        raw: Vec<serde_json::Value>,
        version: ManifestVersion,
    ) -> Vec<WebAccessibleResourceConfig> {
        let mut res = Vec::new();
        for val in raw {
            match version {
                ManifestVersion::V2 => {
                    if let Some(s) = val.as_str() {
                        res.push(WebAccessibleResourceConfig {
                            resources: vec![s.to_string()],
                            matches: vec![],
                            extension_ids: vec![],
                            use_dynamic_url: None,
                        });
                    }
                }
                ManifestVersion::V3 => {
                    if let Some(obj) = val.as_object() {
                        let mut resources = Vec::new();
                        if let Some(r) = obj.get("resources").and_then(|v| v.as_array()) {
                            for v in r {
                                if let Some(s) = v.as_str() {
                                    resources.push(s.to_string());
                                }
                            }
                        }
                        let mut matches = Vec::new();
                        if let Some(m) = obj.get("matches").and_then(|v| v.as_array()) {
                            for v in m {
                                if let Some(s) = v.as_str() {
                                    matches.push(s.to_string());
                                }
                            }
                        }
                        let mut extension_ids = Vec::new();
                        if let Some(e) = obj.get("extension_ids").and_then(|v| v.as_array()) {
                            for v in e {
                                if let Some(s) = v.as_str() {
                                    extension_ids.push(s.to_string());
                                }
                            }
                        }
                        let use_dynamic_url = obj.get("use_dynamic_url").and_then(|v| v.as_bool());

                        res.push(WebAccessibleResourceConfig {
                            resources,
                            matches,
                            extension_ids,
                            use_dynamic_url,
                        });
                    }
                }
            }
        }
        res
    }

    fn map_content_security_policy(
        raw: &Option<serde_json::Value>,
        version: ManifestVersion,
    ) -> Option<crate::domain::entities::ContentSecurityPolicy> {
        let val = raw.as_ref()?;
        match version {
            ManifestVersion::V2 => {
                val.as_str()
                    .map(|s| crate::domain::entities::ContentSecurityPolicy {
                        extension_pages: None,
                        sandbox: None,
                        isolated_world: None,
                        raw_policy: Some(s.to_string()),
                    })
            }
            ManifestVersion::V3 => {
                if let Some(obj) = val.as_object() {
                    let extension_pages = obj
                        .get("extension_pages")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let sandbox = obj
                        .get("sandbox")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let isolated_world = obj
                        .get("isolated_world")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    Some(crate::domain::entities::ContentSecurityPolicy {
                        extension_pages,
                        sandbox,
                        isolated_world,
                        raw_policy: None,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn map_externally_connectable(
        raw: &Option<serde_json::Value>,
    ) -> Option<crate::domain::entities::ExternallyConnectable> {
        let obj = raw.as_ref()?.as_object()?;

        let mut ids = Vec::new();
        if let Some(i) = obj.get("ids").and_then(|v| v.as_array()) {
            for v in i {
                if let Some(s) = v.as_str() {
                    ids.push(s.to_string());
                }
            }
        }

        let mut matches = Vec::new();
        if let Some(m) = obj.get("matches").and_then(|v| v.as_array()) {
            for v in m {
                if let Some(s) = v.as_str() {
                    matches.push(s.to_string());
                }
            }
        }

        let accepts_tls_channel_id = obj.get("accepts_tls_channel_id").and_then(|v| v.as_bool());

        Some(crate::domain::entities::ExternallyConnectable {
            ids,
            matches,
            accepts_tls_channel_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_map_csp_v2() {
        let csp_val = Some(json!("script-src 'self'; object-src 'self'"));
        let csp =
            ManifestMapper::map_content_security_policy(&csp_val, ManifestVersion::V2).unwrap();
        assert_eq!(
            csp.raw_policy,
            Some("script-src 'self'; object-src 'self'".to_string())
        );
        assert_eq!(csp.extension_pages, None);
    }

    #[test]
    fn test_map_csp_v3() {
        let csp_val = Some(json!({
            "extension_pages": "script-src 'self'; object-src 'self'",
            "sandbox": "sandbox allow-scripts"
        }));
        let csp =
            ManifestMapper::map_content_security_policy(&csp_val, ManifestVersion::V3).unwrap();
        assert_eq!(
            csp.extension_pages,
            Some("script-src 'self'; object-src 'self'".to_string())
        );
        assert_eq!(csp.sandbox, Some("sandbox allow-scripts".to_string()));
        assert_eq!(csp.raw_policy, None);
    }

    #[test]
    fn test_map_externally_connectable() {
        let val = Some(json!({
            "ids": ["abcdefghijklmnopabcdefghijklmnop"],
            "matches": ["*://*.example.com/*"],
            "accepts_tls_channel_id": false
        }));
        let ec = ManifestMapper::map_externally_connectable(&val).unwrap();
        assert_eq!(ec.ids.len(), 1);
        assert_eq!(ec.ids[0], "abcdefghijklmnopabcdefghijklmnop");
        assert_eq!(ec.matches.len(), 1);
        assert_eq!(ec.matches[0], "*://*.example.com/*");
        assert_eq!(ec.accepts_tls_channel_id, Some(false));
    }
}
