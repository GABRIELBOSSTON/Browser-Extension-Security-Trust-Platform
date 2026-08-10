use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

pub struct ChromeApiDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
}

impl Default for ChromeApiDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeApiDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
        }
    }

    fn check_chrome_api(&self, name: &str) -> Option<(&'static str, String)> {
        let api_path = if let Some(stripped) = name.strip_prefix("chrome.") {
            stripped
        } else {
            name.strip_prefix("browser.")?
        };

        if api_path.starts_with("cookies") {
            Some((
                "High",
                format!(
                    "Accesses {} API, which can read sensitive session data",
                    name
                ),
            ))
        } else if api_path.starts_with("history") {
            Some((
                "Medium",
                format!("Accesses {} API, which reads browsing history", name),
            ))
        } else if api_path.starts_with("tabs") {
            Some((
                "Medium",
                format!("Accesses {} API, which can interact with open tabs", name),
            ))
        } else if api_path.starts_with("management") {
            Some((
                "High",
                format!("Accesses {} API, which can manage other extensions", name),
            ))
        } else if api_path.starts_with("debugger") {
            Some((
                "Critical",
                format!(
                    "Accesses {} API, which can intercept network traffic and execution",
                    name
                ),
            ))
        } else if api_path.starts_with("proxy") {
            Some((
                "High",
                format!(
                    "Accesses {} API, which controls browser proxy settings",
                    name
                ),
            ))
        } else if api_path.starts_with("webRequest") {
            Some((
                "High",
                format!(
                    "Accesses {} API, which can intercept and block network requests",
                    name
                ),
            ))
        } else if api_path.starts_with("scripting") {
            Some((
                "Medium",
                format!("Accesses {} API, which can inject scripts into pages", name),
            ))
        } else {
            // General Chrome API usage
            Some(("Low", format!("Uses {} API", name)))
        }
    }
}

impl AstDetector for ChromeApiDetector {
    fn detector_id(&self) -> &str {
        "DET-CHROME-API-002"
    }
    fn detector_name(&self) -> &str {
        "Chrome API Usage Detector"
    }

    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[AstNodeKind::CallExpression, AstNodeKind::MemberExpression]
    }

    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext) {
        let callee = context
            .metadata
            .get("callee_name")
            .cloned()
            .or_else(|| context.metadata.get("expression_text").cloned());

        if let Some(raw_name) = callee {
            if let Some((severity, reason)) = self.check_chrome_api(&raw_name) {
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

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-CHROME-{}", self.findings.len()),
                    node_kind: node.clone(),
                    location: SourceLocation {
                        line,
                        column,
                        start_offset: 0,
                        end_offset: 0,
                    },
                    message: reason.to_string(),
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
