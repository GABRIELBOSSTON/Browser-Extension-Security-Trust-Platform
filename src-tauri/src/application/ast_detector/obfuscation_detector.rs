use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

/// Detects Obfuscation patterns:
/// atob, btoa, unescape, fromCharCode, charCodeAt
pub struct ObfuscationDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
}

impl Default for ObfuscationDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ObfuscationDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
        }
    }

    fn check_obfuscation(name: &str) -> Option<(&'static str, &'static str)> {
        match name {
            "atob" => Some((
                "High",
                "Obfuscation: atob() decodes Base64 — commonly used to hide malicious payloads",
            )),
            "btoa" => Some((
                "Medium",
                "Obfuscation: btoa() encodes to Base64 — may be used to obscure data before exfiltration",
            )),
            "unescape" => Some((
                "High",
                "Obfuscation: unescape() is deprecated and often used to decode obfuscated code",
            )),
            "String.fromCharCode" | "fromCharCode" => Some((
                "High",
                "Obfuscation: fromCharCode() is a classic technique to hide strings from static analysis",
            )),
            "charCodeAt" => Some((
                "Medium",
                "Obfuscation: charCodeAt() converts characters to numeric codes — may be part of obfuscation",
            )),
            _ => {
                if name.ends_with(".fromCharCode") {
                    Some((
                        "High",
                        "Obfuscation: fromCharCode() is a classic technique to hide strings from static analysis",
                    ))
                } else if name.ends_with(".charCodeAt") {
                    Some((
                        "Medium",
                        "Obfuscation: charCodeAt() converts characters to numeric codes",
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl AstDetector for ObfuscationDetector {
    fn detector_id(&self) -> &str {
        "DET-OBFUSC-001"
    }

    fn detector_name(&self) -> &str {
        "Obfuscation Detector"
    }

    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[
            AstNodeKind::CallExpression,
            AstNodeKind::MemberExpression,
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
            if let Some((severity, reason)) = Self::check_obfuscation(&raw_name) {
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
                metadata.insert("category".to_string(), "Obfuscation".to_string());

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-OBFUSC-{}", self.findings.len()),
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

    fn make_context(callee: &str, line: usize) -> DetectorContext {
        let mut ctx = DetectorContext::default();
        ctx.metadata
            .insert("callee_name".to_string(), callee.to_string());
        ctx.metadata.insert("line".to_string(), line.to_string());
        ctx.metadata.insert("column".to_string(), "0".to_string());
        ctx
    }

    #[test]
    fn test_atob_is_high() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("atob", 3);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("atob()"));
    }

    #[test]
    fn test_btoa_is_medium() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("btoa", 4);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Medium");
    }

    #[test]
    fn test_unescape_is_high() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("unescape", 7);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("unescape()"));
    }

    #[test]
    fn test_from_char_code_is_high() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("String.fromCharCode", 2);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("fromCharCode()"));
    }

    #[test]
    fn test_char_code_at_is_medium() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("charCodeAt", 1);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Medium");
    }

    #[test]
    fn test_member_expr_from_char_code() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("str.fromCharCode", 5);
        det.enter(AstNodeKind::MemberExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
    }

    #[test]
    fn test_safe_call_not_flagged() {
        let mut det = ObfuscationDetector::new();
        let mut ctx = make_context("JSON.parse", 1);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 0);
    }
}
