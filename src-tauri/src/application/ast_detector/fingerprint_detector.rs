use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

/// Detects Browser Fingerprinting patterns:
/// canvas (toDataURL / getImageData), AudioContext,
/// navigator.hardwareConcurrency, navigator.deviceMemory, navigator.plugins
pub struct FingerprintDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
}

impl Default for FingerprintDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl FingerprintDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
        }
    }

    fn check_fingerprint(name: &str) -> Option<(&'static str, &'static str)> {
        match name {
            "toDataURL" | "canvas.toDataURL" | "getImageData" | "canvas.getImageData" => Some((
                "High",
                "Fingerprinting: Canvas API used to generate a unique browser fingerprint",
            )),
            "AudioContext" | "webkitAudioContext" => Some((
                "High",
                "Fingerprinting: AudioContext used to measure audio hardware for fingerprinting",
            )),
            "navigator.hardwareConcurrency" | "hardwareConcurrency" => Some((
                "Medium",
                "Fingerprinting: hardwareConcurrency exposes CPU core count for fingerprinting",
            )),
            "navigator.deviceMemory" | "deviceMemory" => Some((
                "Medium",
                "Fingerprinting: deviceMemory exposes RAM amount for fingerprinting",
            )),
            "navigator.plugins" => Some((
                "Medium",
                "Fingerprinting: navigator.plugins exposes installed plugins for fingerprinting",
            )),
            _ => {
                // Catch any member access pattern that resolves to these names
                if name.ends_with(".toDataURL") || name.ends_with(".getImageData") {
                    Some((
                        "High",
                        "Fingerprinting: Canvas API used to generate a unique browser fingerprint",
                    ))
                } else if name.contains("AudioContext") {
                    Some((
                        "High",
                        "Fingerprinting: AudioContext used to measure audio hardware for fingerprinting",
                    ))
                } else if name.ends_with(".hardwareConcurrency") {
                    Some((
                        "Medium",
                        "Fingerprinting: hardwareConcurrency exposes CPU core count for fingerprinting",
                    ))
                } else if name.ends_with(".deviceMemory") {
                    Some((
                        "Medium",
                        "Fingerprinting: deviceMemory exposes RAM amount for fingerprinting",
                    ))
                } else if name.ends_with(".plugins") || name == "plugins" {
                    Some((
                        "Medium",
                        "Fingerprinting: navigator.plugins exposes installed plugins for fingerprinting",
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl AstDetector for FingerprintDetector {
    fn detector_id(&self) -> &str {
        "DET-FINGERPRINT-001"
    }

    fn detector_name(&self) -> &str {
        "Browser Fingerprinting Detector"
    }

    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[
            AstNodeKind::CallExpression,
            AstNodeKind::NewExpression,
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
            if let Some((severity, reason)) = Self::check_fingerprint(&raw_name) {
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
                metadata.insert("category".to_string(), "Fingerprinting".to_string());

                self.findings.push(DetectionFinding {
                    finding_id: format!("F-FINGERPRINT-{}", self.findings.len()),
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
    fn test_canvas_to_data_url_is_high() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("canvas.toDataURL", 5);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("Canvas"));
    }

    #[test]
    fn test_audio_context_is_high() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("AudioContext", 10);
        det.enter(AstNodeKind::NewExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
        assert!(result.findings[0].message.contains("AudioContext"));
    }

    #[test]
    fn test_hardware_concurrency_is_medium() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("navigator.hardwareConcurrency", 2);
        det.enter(AstNodeKind::MemberExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Medium");
        assert!(result.findings[0].message.contains("hardwareConcurrency"));
    }

    #[test]
    fn test_device_memory_is_medium() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("navigator.deviceMemory", 3);
        det.enter(AstNodeKind::MemberExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Medium");
        assert!(result.findings[0].message.contains("deviceMemory"));
    }

    #[test]
    fn test_navigator_plugins_is_medium() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("navigator.plugins", 8);
        det.enter(AstNodeKind::MemberExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "Medium");
        assert!(result.findings[0].message.contains("plugins"));
    }

    #[test]
    fn test_get_image_data_is_high() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("ctx.getImageData", 12);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].metadata["severity"], "High");
    }

    #[test]
    fn test_safe_call_not_flagged() {
        let mut det = FingerprintDetector::new();
        let mut ctx = make_context("document.getElementById", 1);
        det.enter(AstNodeKind::CallExpression, &mut ctx);
        let result = det.finish(&mut ctx);
        assert_eq!(result.findings.len(), 0);
    }
}
