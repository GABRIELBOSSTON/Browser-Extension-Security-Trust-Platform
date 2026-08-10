use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

pub struct SecretDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
    patterns: Vec<(Regex, &'static str, &'static str)>, // (Regex, Type, Severity)
}

impl Default for SecretDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretDetector {
    pub fn new() -> Self {
        let patterns = vec![
            (
                Regex::new(r#"(?i)(?:key|api[_-]?key|secret|token|password)[^\n]{0,20}['"][0-9a-zA-Z]{16,40}['"]"#).unwrap(),
                "Generic API Key",
                "High",
            ),
            (
                Regex::new(r"ey[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*").unwrap(),
                "JWT Token",
                "High",
            ),
            (
                Regex::new(r"(?i)bearer\s+[A-Za-z0-9\-._~+/]+").unwrap(),
                "Bearer Token",
                "High",
            ),
            (
                Regex::new(r"AIza[0-9A-Za-z\-_]{35}").unwrap(),
                "Google API Key",
                "High",
            ),
            (
                Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
                "AWS Access Key",
                "High",
            ),
            (
                Regex::new(r"-----BEGIN (?:RSA )?PRIVATE KEY-----").unwrap(),
                "Private Key",
                "Critical",
            ),
        ];

        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
            patterns,
        }
    }

    fn check_secret(&self, content: &str) -> Option<(&'static str, &'static str, String)> {
        for (re, secret_type, severity) in &self.patterns {
            if let Some(mat) = re.find(content) {
                return Some((*secret_type, *severity, mat.as_str().to_string()));
            }
        }
        None
    }
}

impl AstDetector for SecretDetector {
    fn detector_id(&self) -> &str {
        "DET-SECRET-002"
    }
    fn detector_name(&self) -> &str {
        "Hardcoded Secret Detector"
    }

    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[AstNodeKind::StringLiteral, AstNodeKind::TemplateLiteral]
    }

    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext) {
        if let Some(content) = context.metadata.get("simulated_string_literal") {
            if let Some((secret_type, severity, matched_str)) = self.check_secret(content) {
                self.visited += 1;

                let line = context
                    .metadata
                    .get("line")
                    .and_then(|l| l.parse().ok())
                    .unwrap_or(0);
                let column = context
                    .metadata
                    .get("column")
                    .and_then(|c| c.parse().ok())
                    .unwrap_or(0);

                let mut metadata = HashMap::new();
                metadata.insert("severity".to_string(), severity.to_string());
                metadata.insert(
                    "filename".to_string(),
                    context.current_file.clone().unwrap_or_default(),
                );
                metadata.insert("secret_type".to_string(), secret_type.to_string());

                // Truncate the match for safety
                let preview = if matched_str.len() > 20 {
                    format!("{}...", &matched_str[..17])
                } else {
                    matched_str
                };

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-SECRET-{}", self.findings.len()),
                    node_kind: node.clone(),
                    location: SourceLocation {
                        line,
                        column,
                        start_offset: 0,
                        end_offset: 0,
                    },
                    message: format!("Hardcoded {}: {}", secret_type, preview),
                    metadata,
                });
            }
        }
    }

    fn leave(&mut self, _node: AstNodeKind, _context: &mut DetectorContext) {}

    fn finish(&mut self, _context: &mut DetectorContext) -> DetectorResult {
        DetectorResult {
            detector_id: self.detector_id().to_string(),
            findings: std::mem::take(&mut self.findings),
            statistics: std::collections::HashMap::new(),
            warnings: Vec::new(),
            elapsed_ms: self.start_time.elapsed().as_millis() as u64,
            visited_nodes: self.visited,
            skipped_nodes: 0,
            cancelled: false,
        }
    }
}
