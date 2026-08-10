use crate::application::ast_detector::scanner::ASTFinding;
use crate::domain::explanation::{Evidence, Recommendation, SecurityExplanation};

/// Input data used to generate a security explanation.
#[derive(Debug, Clone)]
pub struct ExplanationInput {
    pub extension_id: String,
    pub extension_name: String,
    pub risk_score: u32,
    pub risk_level: String,
    /// Manifest-level risk reasons (from ManifestRiskEngine)
    pub manifest_reasons: Vec<String>,
    /// Raw permissions list
    pub permissions: Vec<String>,
    /// Raw host permissions list
    pub host_permissions: Vec<String>,
    /// AST-level findings (from AstScannerService)
    pub ast_findings: Vec<ASTFinding>,
}

/// Rule-template driven explanation engine.
/// Converts structured risk findings into human-readable security reports.
pub struct ExplanationEngine;

impl ExplanationEngine {
    pub fn explain(input: &ExplanationInput) -> SecurityExplanation {
        let evidence = Self::build_evidence(input);
        let potential_impact = Self::build_impact(input);
        let recommendations = Self::build_recommendations(input);
        let summary = Self::build_summary(input, &evidence);

        SecurityExplanation {
            extension_id: input.extension_id.clone(),
            extension_name: input.extension_name.clone(),
            risk_score: input.risk_score,
            risk_level: input.risk_level.clone(),
            summary,
            evidence,
            potential_impact,
            recommendations,
        }
    }

    // ─── Private builders ───────────────────────────────────────────────────

    fn build_summary(input: &ExplanationInput, evidence: &[Evidence]) -> String {
        let level_desc = match input.risk_level.to_lowercase().as_str() {
            "safe" => "appears safe with no significant risks detected",
            "low" => "has a low risk profile with minor concerns",
            "medium" => "carries a moderate risk and warrants attention",
            "high" => "poses a high security risk and should be reviewed carefully",
            "critical" => "poses a CRITICAL security risk and should be disabled immediately",
            _ => "has an undetermined risk profile",
        };

        let critical_count = evidence
            .iter()
            .filter(|e| e.severity.to_lowercase() == "critical")
            .count();
        let high_count = evidence
            .iter()
            .filter(|e| e.severity.to_lowercase() == "high")
            .count();

        let perm_count = input.permissions.len();
        let ast_count = input.ast_findings.len();
        let host_count = input.host_permissions.len();

        let mut parts = vec![format!(
            "The extension \"{}\" (score: {}/100) {}.",
            input.extension_name, input.risk_score, level_desc
        )];

        if perm_count > 0 || host_count > 0 {
            parts.push(format!(
                "It requests {} permission(s) and {} host permission(s).",
                perm_count, host_count
            ));
        }

        if ast_count > 0 {
            parts.push(format!(
                "Static code analysis found {} suspicious code pattern(s).",
                ast_count
            ));
        }

        if critical_count > 0 || high_count > 0 {
            parts.push(format!(
                "{} critical and {} high severity indicators were identified.",
                critical_count, high_count
            ));
        }

        // Specific pattern messages
        let has_stealer = input
            .manifest_reasons
            .iter()
            .any(|r| r.contains("Password Stealer"));
        if has_stealer {
            parts.push(
                "The permission combination (cookies + tabs + webRequest) matches \
                 a known password-stealer fingerprint."
                    .to_string(),
            );
        }

        let has_all_urls = input
            .host_permissions
            .iter()
            .any(|h| h == "<all_urls>" || h == "*://*/*");
        if has_all_urls {
            parts.push(
                "Broad host permissions grant access to all websites, \
                 enabling potential data interception on any page."
                    .to_string(),
            );
        }

        parts.join(" ")
    }

    fn build_evidence(input: &ExplanationInput) -> Vec<Evidence> {
        let mut evidence: Vec<Evidence> = Vec::new();

        // 1. Manifest permission evidence
        for reason in &input.manifest_reasons {
            let severity = Self::classify_reason_severity(reason);
            evidence.push(Evidence {
                category: "Manifest".to_string(),
                detail: reason.clone(),
                severity,
            });
        }

        // 2. Host permissions evidence
        for host in &input.host_permissions {
            let (sev, detail) = match host.as_str() {
                "<all_urls>" | "*://*/*" => (
                    "Critical",
                    format!(
                        "Host permission \"{}\" grants read/write access to every website",
                        host
                    ),
                ),
                h if h.starts_with("http://") => (
                    "High",
                    format!(
                        "Host permission \"{}\" allows intercepting insecure HTTP traffic",
                        h
                    ),
                ),
                h => (
                    "Low",
                    format!(
                        "Host permission \"{}\" grants access to matching origins",
                        h
                    ),
                ),
            };
            evidence.push(Evidence {
                category: "Host Permission".to_string(),
                detail,
                severity: sev.to_string(),
            });
        }

        // 3. AST finding evidence (deduplicated by reason)
        let mut seen_reasons = std::collections::HashSet::new();
        for finding in &input.ast_findings {
            if seen_reasons.insert(finding.reason.clone()) {
                evidence.push(Evidence {
                    category: format!("Code Analysis ({})", finding.node_type),
                    detail: format!(
                        "{} — found in {}:{}",
                        finding.reason, finding.filename, finding.line
                    ),
                    severity: finding.severity.clone(),
                });
            }
        }

        // Sort: Critical first, then High, Medium, Low
        evidence.sort_by_key(|e| Self::severity_order(&e.severity));
        evidence
    }

    fn build_impact(input: &ExplanationInput) -> String {
        let has_stealer = input
            .manifest_reasons
            .iter()
            .any(|r| r.contains("Password Stealer"));
        let has_all_urls = input
            .host_permissions
            .iter()
            .any(|h| h == "<all_urls>" || h == "*://*/*");
        let has_eval = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("eval()") || f.reason.contains("Function()"));
        let has_network = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("fetch()") || f.reason.contains("XMLHttpRequest"));
        let has_fingerprint = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("fingerprint") || f.reason.contains("Fingerprint"));
        let has_obfuscation = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("Obfuscation") || f.reason.contains("atob()"));
        let has_native_messaging = input.permissions.iter().any(|p| p == "nativeMessaging");
        let has_debugger = input.permissions.iter().any(|p| p == "debugger");

        let mut impacts: Vec<&str> = Vec::new();

        if has_stealer {
            impacts.push(
                "Session hijacking: By combining cookie access with request interception, \
                 an attacker could steal active login sessions for any website you visit.",
            );
        }
        if has_eval && has_network {
            impacts.push(
                "Remote code execution: The extension can download and execute arbitrary \
                 JavaScript from remote servers, enabling complete browser takeover.",
            );
        } else if has_eval {
            impacts.push(
                "Code injection: Dynamic code execution via eval() or Function() \
                 can be exploited to run malicious payloads inside the browser.",
            );
        }
        if has_all_urls && has_network {
            impacts.push(
                "Data exfiltration: With access to all URLs and network APIs, \
                 the extension could silently upload your browsing history, \
                 form inputs, and credentials to a remote server.",
            );
        }
        if has_fingerprint {
            impacts.push(
                "Browser fingerprinting: Canvas, AudioContext, and hardware APIs \
                 can uniquely identify your device across websites, \
                 enabling persistent cross-site tracking even in private browsing mode.",
            );
        }
        if has_obfuscation {
            impacts.push(
                "Hidden payload: Base64 decoding and character-code manipulation \
                 are classic techniques to hide malicious code from security scanners.",
            );
        }
        if has_native_messaging {
            impacts.push(
                "Host system access: nativeMessaging can communicate with local \
                 programs, potentially enabling file system access or OS-level attacks.",
            );
        }
        if has_debugger {
            impacts.push(
                "Full browser control: The debugger API grants the ability to pause \
                 JavaScript execution, inspect memory, and intercept any network request \
                 across all open tabs.",
            );
        }

        if impacts.is_empty() {
            match input.risk_level.to_lowercase().as_str() {
                "safe" => "No significant impact identified. \
                     The extension follows standard permission practices."
                    .to_string(),
                "low" => "Minimal risk. The extension has limited access capabilities \
                          that pose little practical threat."
                    .to_string(),
                _ => "The identified permissions may allow the extension to access \
                     sensitive browser data beyond what is typical for its stated purpose."
                    .to_string(),
            }
        } else {
            impacts.join("\n\n")
        }
    }

    fn build_recommendations(input: &ExplanationInput) -> Vec<Recommendation> {
        let mut recs: Vec<Recommendation> = Vec::new();
        let mut priority: u8 = 1;

        // Critical action first
        if matches!(
            input.risk_level.to_lowercase().as_str(),
            "critical" | "high"
        ) {
            recs.push(Recommendation {
                action: "Disable or Remove Immediately".to_string(),
                description: format!(
                    "Given the {} risk score of {}/100, consider disabling \"{}\" \
                     from your browser's extension manager until you can verify its \
                     legitimacy with the publisher.",
                    input.risk_level, input.risk_score, input.extension_name
                ),
                priority,
            });
            priority += 1;
        }

        // Password stealer pattern
        let has_stealer = input
            .manifest_reasons
            .iter()
            .any(|r| r.contains("Password Stealer"));
        if has_stealer {
            recs.push(Recommendation {
                action: "Change Passwords for Critical Accounts".to_string(),
                description: "If this extension was active while you were logged into banking, \
                     email, or social media sites, change those passwords as a precaution. \
                     The cookies + tabs + webRequest combination can intercept active sessions."
                    .to_string(),
                priority,
            });
            priority += 1;
        }

        // eval / RCE
        let has_eval = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("eval()") || f.reason.contains("Function()"));
        if has_eval {
            recs.push(Recommendation {
                action: "Review Extension Source Code".to_string(),
                description: "Dynamic code execution (eval / new Function) was detected. \
                     Review the extension's source code on the Chrome Web Store or \
                     GitHub to understand why it uses these patterns. \
                     Legitimate extensions rarely need eval()."
                    .to_string(),
                priority,
            });
            priority += 1;
        }

        // Obfuscation
        let has_obfuscation = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("Obfuscation") || f.reason.contains("atob()"));
        if has_obfuscation {
            recs.push(Recommendation {
                action: "Report to Browser Store".to_string(),
                description:
                    "Code obfuscation was detected. Legitimate extensions typically do not \
                     hide their code. Consider reporting this extension to the Chrome Web Store \
                     or Microsoft Edge Add-ons for review."
                        .to_string(),
                priority,
            });
            priority += 1;
        }

        // All URLs
        let has_all_urls = input
            .host_permissions
            .iter()
            .any(|h| h == "<all_urls>" || h == "*://*/*");
        if has_all_urls {
            recs.push(Recommendation {
                action: "Verify Purpose Requires All-URL Access".to_string(),
                description: format!(
                    "\"{}\" requests access to all websites. \
                     Confirm this is necessary for the extension's stated purpose. \
                     If it is a simple utility (e.g., a theme or spell checker), \
                     this permission scope is excessive.",
                    input.extension_name
                ),
                priority,
            });
            priority += 1;
        }

        // Network exfiltration
        let has_network = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("Exfiltration"));
        if has_network && has_all_urls {
            recs.push(Recommendation {
                action: "Monitor Network Traffic".to_string(),
                description: "Use browser DevTools (Network tab) while the extension is active \
                     to inspect outbound HTTP requests. Look for requests to unknown domains \
                     that transmit personal data."
                    .to_string(),
                priority,
            });
            priority += 1;
        }

        // Fingerprinting
        let has_fingerprint = input
            .ast_findings
            .iter()
            .any(|f| f.reason.contains("Fingerprint") || f.reason.contains("fingerprint"));
        if has_fingerprint {
            recs.push(Recommendation {
                action: "Use a Privacy-Focused Browser Profile".to_string(),
                description: "This extension uses browser fingerprinting APIs. \
                     Consider running it in a separate browser profile or \
                     a sandboxed browser to prevent cross-site tracking."
                    .to_string(),
                priority,
            });
            priority += 1;
        }

        // Generic fallback
        if recs.is_empty() {
            recs.push(Recommendation {
                action: "Continue Monitoring".to_string(),
                description: format!(
                    "\"{}\" currently presents low risk. \
                     Re-scan after extension updates to catch any new suspicious behavior.",
                    input.extension_name
                ),
                priority,
            });
        }

        recs
    }

    // ─── Helpers ────────────────────────────────────────────────────────────

    fn classify_reason_severity(reason: &str) -> String {
        let lower = reason.to_lowercase();
        if lower.contains("password stealer")
            || lower.contains("nativemessaging")
            || lower.contains("debugger")
            || lower.contains("<all_urls>")
            || lower.contains("*://*/*")
        {
            "Critical".to_string()
        } else if lower.contains("proxy")
            || lower.contains("cookies")
            || lower.contains("management")
            || lower.contains("webrequest")
        {
            "High".to_string()
        } else if lower.contains("history")
            || lower.contains("tabs")
            || lower.contains("unsafe-inline")
            || lower.contains("unsafe-eval")
        {
            "Medium".to_string()
        } else {
            "Low".to_string()
        }
    }

    fn severity_order(severity: &str) -> u8 {
        match severity.to_lowercase().as_str() {
            "critical" => 0,
            "high" => 1,
            "medium" => 2,
            "low" => 3,
            _ => 4,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unit Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn base_input() -> ExplanationInput {
        ExplanationInput {
            extension_id: "test-ext-001".to_string(),
            extension_name: "Test Extension".to_string(),
            risk_score: 0,
            risk_level: "Safe".to_string(),
            manifest_reasons: vec![],
            permissions: vec![],
            host_permissions: vec![],
            ast_findings: vec![],
        }
    }

    #[test]
    fn test_safe_extension_explanation() {
        let input = base_input();
        let result = ExplanationEngine::explain(&input);

        assert_eq!(result.risk_level, "Safe");
        assert!(result.summary.contains("appears safe"));
        assert!(result.evidence.is_empty());
        assert!(!result.potential_impact.is_empty());
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_critical_permission_generates_evidence() {
        let mut input = base_input();
        input.risk_score = 40;
        input.risk_level = "Low".to_string();
        input.manifest_reasons = vec!["nativeMessaging".to_string()];
        input.permissions = vec!["nativeMessaging".to_string()];

        let result = ExplanationEngine::explain(&input);

        assert!(!result.evidence.is_empty());
        let critical_ev = result
            .evidence
            .iter()
            .find(|e| e.detail.contains("nativeMessaging"));
        assert!(critical_ev.is_some());
        assert_eq!(critical_ev.unwrap().severity, "Critical");
    }

    #[test]
    fn test_password_stealer_pattern_in_summary() {
        let mut input = base_input();
        input.risk_score = 80;
        input.risk_level = "High".to_string();
        input.manifest_reasons = vec![
            "cookies".to_string(),
            "tabs".to_string(),
            "Password Stealer Pattern (cookies + tabs + webRequest)".to_string(),
        ];
        input.permissions = vec![
            "cookies".to_string(),
            "tabs".to_string(),
            "webRequest".to_string(),
        ];

        let result = ExplanationEngine::explain(&input);

        assert!(result.summary.contains("password-stealer fingerprint"));
        assert!(result
            .recommendations
            .iter()
            .any(|r| r.action.contains("Passwords")));
        assert!(result.potential_impact.contains("Session hijacking"));
    }

    #[test]
    fn test_all_urls_host_permission_is_critical_evidence() {
        let mut input = base_input();
        input.risk_score = 60;
        input.risk_level = "Medium".to_string();
        input.host_permissions = vec!["<all_urls>".to_string()];

        let result = ExplanationEngine::explain(&input);

        let host_ev = result
            .evidence
            .iter()
            .find(|e| e.category == "Host Permission");
        assert!(host_ev.is_some());
        assert_eq!(host_ev.unwrap().severity, "Critical");
        assert!(result.summary.contains("Broad host permissions"));
    }

    #[test]
    fn test_ast_finding_generates_evidence() {
        let mut input = base_input();
        input.risk_score = 50;
        input.risk_level = "Medium".to_string();
        input.ast_findings = vec![ASTFinding {
            filename: "content.js".to_string(),
            line: 42,
            column: 5,
            severity: "Critical".to_string(),
            reason: "Remote Code Execution: eval() executes arbitrary code".to_string(),
            node_type: "CallExpression".to_string(),
        }];

        let result = ExplanationEngine::explain(&input);

        let ast_ev = result
            .evidence
            .iter()
            .find(|e| e.category.starts_with("Code Analysis"));
        assert!(ast_ev.is_some());
        assert_eq!(ast_ev.unwrap().severity, "Critical");
        assert!(result.potential_impact.contains("Code injection"));
    }

    #[test]
    fn test_high_risk_generates_disable_recommendation() {
        let mut input = base_input();
        input.risk_score = 85;
        input.risk_level = "Critical".to_string();
        input.manifest_reasons = vec!["nativeMessaging".to_string()];

        let result = ExplanationEngine::explain(&input);

        assert!(result
            .recommendations
            .iter()
            .any(|r| r.action.contains("Disable")));
        assert_eq!(
            result
                .recommendations
                .iter()
                .find(|r| r.action.contains("Disable"))
                .unwrap()
                .priority,
            1
        );
    }

    #[test]
    fn test_eval_with_network_means_rce_impact() {
        let mut input = base_input();
        input.risk_score = 70;
        input.risk_level = "High".to_string();
        input.ast_findings = vec![
            ASTFinding {
                filename: "bg.js".to_string(),
                line: 10,
                column: 0,
                severity: "Critical".to_string(),
                reason: "Remote Code Execution: eval() executes arbitrary code".to_string(),
                node_type: "CallExpression".to_string(),
            },
            ASTFinding {
                filename: "bg.js".to_string(),
                line: 8,
                column: 0,
                severity: "High".to_string(),
                reason: "Data Exfiltration: fetch() can send extension data to remote servers"
                    .to_string(),
                node_type: "CallExpression".to_string(),
            },
        ];

        let result = ExplanationEngine::explain(&input);
        assert!(result.potential_impact.contains("Remote code execution"));
    }

    #[test]
    fn test_evidence_sorted_critical_first() {
        let mut input = base_input();
        input.manifest_reasons = vec![
            "tabs".to_string(),            // Medium
            "nativeMessaging".to_string(), // Critical
            "cookies".to_string(),         // High
        ];

        let result = ExplanationEngine::explain(&input);
        assert!(!result.evidence.is_empty());
        // First evidence should be Critical
        assert_eq!(result.evidence[0].severity, "Critical");
    }

    #[test]
    fn test_obfuscation_recommends_report_to_store() {
        let mut input = base_input();
        input.risk_score = 55;
        input.risk_level = "Medium".to_string();
        input.ast_findings = vec![ASTFinding {
            filename: "inject.js".to_string(),
            line: 1,
            column: 0,
            severity: "High".to_string(),
            reason: "Obfuscation: atob() decodes Base64 — commonly used to hide malicious payloads"
                .to_string(),
            node_type: "CallExpression".to_string(),
        }];

        let result = ExplanationEngine::explain(&input);
        assert!(result
            .recommendations
            .iter()
            .any(|r| r.action.contains("Report")));
    }
}
