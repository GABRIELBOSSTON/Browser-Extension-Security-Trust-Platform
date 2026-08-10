use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

/// Detects Data Exfiltration patterns:
/// Network APIs (fetch, XHR, sendBeacon, WebSocket) combined with broad host permissions
/// are flagged at the AST level. Host-permission cross-checking is left to ManifestRiskEngine.
pub struct DataExfiltrationDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
}

impl Default for DataExfiltrationDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DataExfiltrationDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
        }
    }

    fn check_network_api(name: &str) -> Option<(&'static str, &'static str)> {
        // Exact callee names we care about
        match name {
            "fetch" => Some((
                "High",
                "Data Exfiltration: fetch() can send extension data to remote servers",
            )),
            "XMLHttpRequest" | "XmlHttpRequest" => Some((
                "High",
                "Data Exfiltration: XMLHttpRequest can send data to remote servers",
            )),
            "navigator.sendBeacon" | "sendBeacon" => Some((
                "High",
                "Data Exfiltration: sendBeacon() silently sends data on page unload",
            )),
            "WebSocket" => Some((
                "High",
                "Data Exfiltration: WebSocket enables persistent data channel to remote server",
            )),
            _ => {
                if name.contains("sendBeacon") {
                    Some((
                        "High",
                        "Data Exfiltration: sendBeacon() silently sends data on page unload",
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl AstDetector for DataExfiltrationDetector {
    fn detector_id(&self) -> &str {
        "DET-EXFIL-001"
    }

    fn detector_name(&self) -> &str {
        "Data Exfiltration Detector"
    }

    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[
            AstNodeKind::CallExpression,
            AstNodeKind::NewExpression,
            AstNodeKind::Identifier,
        ]
    }

    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext) {
        let callee = context
            .metadata
            .get("callee_name")
            .cloned()
            .or_else(|| context.metadata.get("expression_text").cloned());

        if let Some(raw_name) = callee {
            if let Some((severity, reason)) = Self::check_network_api(&raw_name) {
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
                metadata.insert("category".to_string(), "DataExfiltration".to_string());

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-EXFIL-{}", self.findings.len()),
                    node_kind: node,
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
            statistics: HashMap::new(),
            warnings: Vec::new(),
            elapsed_ms: self.start_time.elapsed().as_millis() as u64,
            visited_nodes: self.visited,
            skipped_nodes: 0,
            cancelled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_context(callee: &str, line: usize, col: usize) -> DetectorContext {
        let mut ctx = DetectorContext::default();
        ctx.metadata
            .insert("callee_name".to_string(), callee.to_string());
        ctx.metadata.insert("line".to_string(), line.to_string());
        ctx.metadata.insert("column".to_string(), col.to_string());
        ctx
    }

    #[test]
    fn test_fetch_detected_as_exfiltration() {
        let mut det = DataExfiltrationDetector::new();
        let mut ctx = make_context("fetch", 10, 5);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("fetch()"));
    }

    #[test]
    fn test_xmlhttprequest_detected() {
        let mut det = DataExfiltrationDetector::new();
        let mut ctx = make_context("XMLHttpRequest", 5, 0);
        det.enter(AstNodeKind::NewExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert!(result.findings[0].message.contains("XMLHttpRequest"));
    }

    #[test]
    fn test_sendbeacon_detected() {
        let mut det = DataExfiltrationDetector::new();
        let mut ctx = make_context("navigator.sendBeacon", 3, 2);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert!(result.findings[0].message.contains("sendBeacon()"));
    }

    #[test]
    fn test_websocket_detected() {
        let mut det = DataExfiltrationDetector::new();
        let mut ctx = make_context("WebSocket", 1, 0);
        det.enter(AstNodeKind::NewExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert!(result.findings[0].message.contains("WebSocket"));
    }

    #[test]
    fn test_safe_call_not_flagged() {
        let mut det = DataExfiltrationDetector::new();
        let mut ctx = make_context("console.log", 1, 0);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 0);
    }
}
