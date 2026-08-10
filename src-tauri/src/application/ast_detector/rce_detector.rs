use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

/// Detects Remote Code Execution patterns:
/// eval(), new Function(), setTimeout(string), setInterval(string),
/// fetch(...).then(eval)
pub struct RceDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
    /// Tracks if we last saw fetch/XHR so we can flag .then(eval) chain
    last_was_network: bool,
}

impl Default for RceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl RceDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
            last_was_network: false,
        }
    }

    fn check_rce(name: &str, last_was_network: bool) -> Option<(&'static str, &'static str)> {
        match name {
            "eval" => {
                if last_was_network {
                    Some((
                        "Critical",
                        "Remote Code Execution: fetch().then(eval) - executes remote response as code",
                    ))
                } else {
                    Some((
                        "High",
                        "Remote Code Execution: eval() executes arbitrary code",
                    ))
                }
            }
            "Function" => Some((
                "Low",
                "Remote Code Execution: new Function() dynamically constructs and executes code",
            )),
            "setTimeout" => Some((
                "Low",
                "Remote Code Execution: setTimeout(string) executes a string as code",
            )),
            "setInterval" => Some((
                "Low",
                "Remote Code Execution: setInterval(string) executes a string as code",
            )),
            _ => None,
        }
    }

    fn is_network_call(name: &str) -> bool {
        matches!(
            name,
            "fetch" | "XMLHttpRequest" | "axios" | "$.ajax" | "$.get" | "$.post"
        )
    }
}

impl AstDetector for RceDetector {
    fn detector_id(&self) -> &str {
        "DET-RCE-001"
    }

    fn detector_name(&self) -> &str {
        "Remote Code Execution Detector"
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
            // Track whether the callee is a network call to detect .then(eval) chaining
            if Self::is_network_call(&raw_name) {
                self.last_was_network = true;
                return;
            }

            if let Some((severity, reason)) = Self::check_rce(&raw_name, self.last_was_network) {
                self.visited += 1;
                self.last_was_network = false;

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
                metadata.insert("category".to_string(), "RemoteCodeExecution".to_string());

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-RCE-{}", self.findings.len()),
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
            } else {
                self.last_was_network = false;
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
    fn test_eval_is_high() {
        let mut det = RceDetector::new();
        let mut ctx = make_context("eval", 5, 0);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("eval()"));
    }

    #[test]
    fn test_new_function_is_low() {
        let mut det = RceDetector::new();
        let mut ctx = make_context("Function", 3, 0);
        det.enter(AstNodeKind::NewExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Low");
        assert!(result.findings[0].message.contains("new Function()"));
    }

    #[test]
    fn test_settimeout_is_low() {
        let mut det = RceDetector::new();
        let mut ctx = make_context("setTimeout", 7, 4);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Low");
    }

    #[test]
    fn test_setinterval_is_low() {
        let mut det = RceDetector::new();
        let mut ctx = make_context("setInterval", 9, 0);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Low");
    }

    #[test]
    fn test_fetch_then_eval_is_critical_rce() {
        let mut det = RceDetector::new();
        // Simulate: fetch() followed by eval() in AST traversal order
        let mut fetch_ctx = make_context("fetch", 1, 0);
        det.enter(AstNodeKind::CallExpression, &mut fetch_ctx);

        let mut eval_ctx = make_context("eval", 1, 20);
        det.enter(AstNodeKind::CallExpression, &mut eval_ctx);

        let result = det.finish(&mut eval_ctx);
        assert_eq!(result.findings.len(), 1);
        assert!(result.findings[0].message.contains("fetch().then(eval)"));
        assert_eq!(result.findings[0].metadata["severity"], "Critical");
    }

    #[test]
    fn test_safe_function_not_flagged() {
        let mut det = RceDetector::new();
        let mut ctx = make_context("Math.random", 1, 0);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 0);
    }
}
