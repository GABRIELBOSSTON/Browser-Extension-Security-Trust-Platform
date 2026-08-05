use crate::domain::capabilities::*;
use crate::domain::entities::Manifest;
use crate::domain::errors::Result;
use crate::application::analysis::models::{CapabilityAnalysisResult, CapabilityStatistics};

pub struct CapabilityBuilder;

impl CapabilityBuilder {
    pub fn build(manifest: &Manifest) -> Result<CapabilityAnalysisResult> {
        let mut warnings = Vec::new();
        let mut normalization_notes = Vec::new();

        let mut host_patterns = Vec::new();
        let mut api_permissions = Vec::new();

        // Normalize MV2 vs MV3 permission merging
        let mut all_raw_permissions = Vec::new();
        all_raw_permissions.extend(manifest.permissions.items.iter().map(|p| p.permission_string.clone()));
        all_raw_permissions.extend(manifest.host_permissions.items.iter().map(|p| p.permission_string.clone()));

        for raw in all_raw_permissions {
            if raw == "<all_urls>" || raw.contains("://") {
                host_patterns.push(MatchPattern::parse(&raw));
                normalization_notes.push(format!("Extracted host pattern: {}", raw));
            } else {
                api_permissions.push(PermissionCapability {
                    id: CapabilityId(raw.clone()),
                    name: raw,
                    optional: false,
                });
            }
        }

        let mut optional_api_permissions = Vec::new();
        for raw in manifest.optional_permissions.items.iter().map(|p| p.permission_string.clone()) {
            if raw == "<all_urls>" || raw.contains("://") {
                host_patterns.push(MatchPattern::parse(&raw));
                normalization_notes.push(format!("Extracted optional host pattern: {}", raw));
            } else {
                optional_api_permissions.push(PermissionCapability {
                    id: CapabilityId(raw.clone()),
                    name: raw,
                    optional: true,
                });
            }
        }
        api_permissions.extend(optional_api_permissions);

        let permissions = PermissionCapabilities { items: api_permissions };
        let hosts = HostCapabilities { patterns: host_patterns };
        
        let background = BackgroundCapabilities { config: manifest.background.clone() };
        let csp = CSPCapabilities { raw_policy: None }; // Placeholder for future CSP parser
        let actions = ActionCapabilities { config: manifest.action.clone() };
        let content_scripts = ContentScriptCapabilities { scripts: manifest.content_scripts.clone() };
        let web_accessible_resources = WebAccessibleCapabilities { resources: manifest.web_accessible_resources.clone() };

        let stats = CapabilityStatistics {
            total_permissions: permissions.items.len(),
            total_hosts: hosts.patterns.len(),
            total_content_scripts: content_scripts.scripts.len(),
        };

        if stats.total_hosts > 10 {
            warnings.push("High volume of host permissions requested".to_string());
        }

        let model = ExtensionCapabilityModel {
            permissions,
            hosts,
            background,
            csp,
            actions,
            content_scripts,
            web_accessible_resources,
        };

        Ok(CapabilityAnalysisResult {
            model,
            stats,
            warnings,
            normalization_notes,
        })
    }
}
