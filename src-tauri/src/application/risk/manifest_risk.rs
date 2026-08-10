use crate::domain::entities::{BackgroundConfig, Manifest, ManifestVersion};
use crate::domain::evidence::EvidenceItem;
use crate::domain::manifest_risk::{ManifestRiskScore, RiskCategory};

pub struct ManifestRiskEngine;

impl ManifestRiskEngine {
    pub fn analyze(manifest: &Manifest) -> ManifestRiskScore {
        let mut score: u32 = 0;
        let mut findings = Vec::new();

        let mut add_score = |points: u32, reason: &str| {
            score += points;

            let severity = match points {
                0..=10 => "Low",
                11..=30 => "Medium",
                31..=50 => "High",
                _ => "Critical",
            }
            .to_string();

            findings.push(EvidenceItem {
                category: "Manifest".to_string(),
                detail: reason.to_string(),
                severity,
                base_score: points as i32,
            });
        };

        let mut has_cookies = false;
        let mut has_tabs = false;
        let mut has_web_request = false;

        // 1. Permissions
        for perm in &manifest.permissions.items {
            let perm_str = perm.permission_string.as_str();

            if perm_str == "cookies" {
                has_cookies = true;
            }
            if perm_str == "tabs" {
                has_tabs = true;
            }
            if perm_str == "webRequest" || perm_str == "webRequestBlocking" {
                has_web_request = true;
            }

            match perm_str {
                "<all_urls>" | "*://*/*" => add_score(40, perm_str),
                "nativeMessaging" | "debugger" => add_score(40, perm_str),
                "proxy" | "cookies" | "management" | "webRequestBlocking" => {
                    add_score(20, perm_str)
                }
                "history" | "tabs" | "downloads" | "scripting" => add_score(10, perm_str),
                _ => {}
            }
        }

        // Stealer pattern detection
        if has_cookies && has_tabs && has_web_request {
            add_score(50, "Password Stealer Pattern (cookies + tabs + webRequest)");
        }

        // 2. Host Permissions
        for host in &manifest.host_permissions.items {
            match host.permission_string.as_str() {
                "<all_urls>" | "*://*/*" => add_score(40, &host.permission_string),
                "http://*/*" => add_score(20, &host.permission_string),
                _ => {}
            }
        }

        // 3. Manifest Version & CSP
        if manifest.manifest_version == ManifestVersion::V2 {
            add_score(5, "Manifest V2");
        }

        if let Some(csp) = &manifest.content_security_policy {
            let policy_str = csp
                .raw_policy
                .clone()
                .or_else(|| csp.extension_pages.clone())
                .unwrap_or_default();

            if policy_str.contains("unsafe-inline") {
                add_score(5, "unsafe-inline");
            }
            if policy_str.contains("unsafe-eval") {
                add_score(5, "unsafe-eval");
            }
        } else {
            add_score(5, "No CSP");
        }

        // 4. Background
        if let Some(BackgroundConfig::V2 { persistent, .. }) = &manifest.background {
            if persistent.unwrap_or(false) {
                add_score(5, "Persistent background");
            }
        }

        // 5. Content Scripts
        for cs in &manifest.content_scripts {
            for match_url in &cs.matches {
                if match_url == "<all_urls>" {
                    add_score(5, "Inject to <all_urls>");
                    break;
                }
            }
        }

        // Cap score at 100
        let final_score = score.min(100);

        let category = match final_score {
            0..=20 => RiskCategory::Safe,
            21..=40 => RiskCategory::Low,
            41..=60 => RiskCategory::Medium,
            61..=80 => RiskCategory::High,
            _ => RiskCategory::Critical,
        };

        ManifestRiskScore {
            score: final_score,
            category,
            findings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::ManifestMetadata;
    use crate::domain::entities::{
        ContentScriptConfig, ContentSecurityPolicy, Permission, PermissionSet,
    };
    use crate::domain::types::PermissionType;

    fn base_manifest() -> Manifest {
        Manifest {
            manifest_version: ManifestVersion::V3,
            metadata: ManifestMetadata {
                version: "1.0".to_string(),
                name: "Test".to_string(),
                description: None,
            },
            permissions: PermissionSet::default(),
            host_permissions: PermissionSet::default(),
            optional_permissions: PermissionSet::default(),
            background: None,
            action: None,
            icons: None,
            content_scripts: vec![],
            web_accessible_resources: vec![],
            content_security_policy: Some(ContentSecurityPolicy {
                extension_pages: Some("script-src 'self'".to_string()),
                sandbox: None,
                isolated_world: None,
                raw_policy: None,
            }),
            externally_connectable: None,
        }
    }

    #[test]
    fn test_safe_manifest() {
        let manifest = base_manifest();
        let result = ManifestRiskEngine::analyze(&manifest);
        assert_eq!(result.score, 0);
        assert_eq!(result.category, RiskCategory::Safe);
        assert!(result.findings.is_empty());
    }

    #[test]
    fn test_critical_permissions() {
        let mut manifest = base_manifest();
        manifest.permissions.items.push(Permission {
            permission_id: "nativeMessaging".to_string(),
            permission_string: "nativeMessaging".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });

        let result = ManifestRiskEngine::analyze(&manifest);
        assert_eq!(result.score, 40);
        assert_eq!(result.category, RiskCategory::Low);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].detail, "nativeMessaging");
    }

    #[test]
    fn test_multiple_penalties_clamping() {
        let mut manifest = base_manifest();
        manifest.manifest_version = ManifestVersion::V2;
        manifest.permissions.items.push(Permission {
            permission_id: "debugger".to_string(),
            permission_string: "debugger".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });
        manifest.host_permissions.items.push(Permission {
            permission_id: "<all_urls>".to_string(),
            permission_string: "<all_urls>".to_string(),
            permission_type: PermissionType::HostPattern,
            weight: 0.0,
        });
        manifest.host_permissions.items.push(Permission {
            permission_id: "*://*/*".to_string(),
            permission_string: "*://*/*".to_string(),
            permission_type: PermissionType::HostPattern,
            weight: 0.0,
        });
        // Score should be: 5 (V2) + 40 (debugger) + 40 (<all_urls>) + 40 (*://*/*) = 125, clamped to 100

        let result = ManifestRiskEngine::analyze(&manifest);
        assert_eq!(result.score, 100);
        assert_eq!(result.category, RiskCategory::Critical);
        assert_eq!(result.findings.len(), 4);
    }

    #[test]
    fn test_csp_and_content_scripts_penalty() {
        let mut manifest = base_manifest();
        manifest.content_security_policy = Some(ContentSecurityPolicy {
            extension_pages: Some("script-src 'self' 'unsafe-inline' 'unsafe-eval'".to_string()),
            sandbox: None,
            isolated_world: None,
            raw_policy: None,
        });
        manifest.content_scripts.push(ContentScriptConfig {
            matches: vec!["<all_urls>".to_string()],
            js: vec![],
            css: vec![],
            run_at: None,
        });

        let result = ManifestRiskEngine::analyze(&manifest);
        // unsafe-inline (5) + unsafe-eval (5) + inject <all_urls> (5) = 15
        assert_eq!(result.score, 15);
        assert_eq!(result.category, RiskCategory::Safe);
        let reasons: Vec<String> = result.findings.into_iter().map(|f| f.detail).collect();
        assert_eq!(
            reasons,
            vec!["unsafe-inline", "unsafe-eval", "Inject to <all_urls>"]
        );
    }

    #[test]
    fn test_password_stealer_pattern() {
        let mut manifest = base_manifest();
        manifest.permissions.items.push(Permission {
            permission_id: "cookies".to_string(),
            permission_string: "cookies".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });
        manifest.permissions.items.push(Permission {
            permission_id: "tabs".to_string(),
            permission_string: "tabs".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });
        manifest.permissions.items.push(Permission {
            permission_id: "webRequest".to_string(),
            permission_string: "webRequest".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });

        let result = ManifestRiskEngine::analyze(&manifest);
        // cookies (20) + tabs (10) + stealer pattern (50) = 80
        assert_eq!(result.score, 80);
        assert_eq!(result.category, RiskCategory::High);
        let reasons: Vec<String> = result.findings.into_iter().map(|f| f.detail).collect();
        assert!(
            reasons.contains(&"Password Stealer Pattern (cookies + tabs + webRequest)".to_string())
        );
    }

    #[test]
    fn test_all_urls_scoring_regression() {
        let mut manifest = base_manifest();
        manifest.permissions.items.push(Permission {
            permission_id: "<all_urls>".to_string(),
            permission_string: "<all_urls>".to_string(),
            permission_type: PermissionType::ChromeApi,
            weight: 0.0,
        });

        let result = ManifestRiskEngine::analyze(&manifest);
        // <all_urls> is explicitly assigned 40 points (High), not 80 points (Critical).
        assert_eq!(result.score, 40);
        assert_eq!(result.category, RiskCategory::Low);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].detail, "<all_urls>");
        assert_eq!(result.findings[0].base_score, 40);
        assert_eq!(result.findings[0].severity, "High");
    }
}
