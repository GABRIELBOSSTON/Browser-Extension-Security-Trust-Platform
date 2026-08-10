use std::collections::HashMap;
use std::time::Instant;

use crate::domain::ast_detector::{
    AstDetector, AstNodeKind, DetectionFinding, DetectorContext, DetectorResult, SourceLocation,
};

pub struct DangerousApiDetector {
    findings: Vec<DetectionFinding>,
    start_time: Instant,
    visited: usize,
}

impl Default for DangerousApiDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DangerousApiDetector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            start_time: Instant::now(),
            visited: 0,
        }
    }

    fn check_api(&self, name: &str) -> Option<(&'static str, &'static str)> {
        match name {
            // Dynamic Code (RCE overlaps removed, handled by RceDetector)
            "import" => Some(("Medium", "Dynamic import() can load remote modules")),
            "document.createElement" => {
                Some(("Medium", "Creates elements dynamically, possibly scripts"))
            }
            "appendChild" => Some(("Medium", "Appends elements dynamically")),
            // Network
            "fetch" => Some(("Info", "Uses fetch() API to make network requests")),
            "XMLHttpRequest" => Some(("Info", "Uses XMLHttpRequest for network requests")),
            "WebSocket" => Some(("Info", "Establishes a WebSocket connection")),
            "EventSource" => Some(("Info", "Uses EventSource for server-sent events")),
            // Storage
            "localStorage" => Some(("Low", "Accesses localStorage")),
            "sessionStorage" => Some(("Low", "Accesses sessionStorage")),
            "indexedDB" => Some(("Low", "Accesses indexedDB")),
            // Fingerprinting
            "navigator.plugins" => Some((
                "Medium",
                "Accesses browser plugins, often used for fingerprinting",
            )),
            "canvas.toDataURL" => Some((
                "Medium",
                "Reads canvas data, often used for canvas fingerprinting",
            )),
            "AudioContext" => Some((
                "Medium",
                "Uses AudioContext, possibly for audio fingerprinting",
            )),
            "WebGLRenderingContext" => Some(("Medium", "Uses WebGL, possibly for fingerprinting")),
            _ => {
                if name.contains("localStorage") {
                    Some(("Low", "Accesses localStorage"))
                } else if name.contains("sessionStorage") {
                    Some(("Low", "Accesses sessionStorage"))
                } else if name.contains("indexedDB") {
                    Some(("Low", "Accesses indexedDB"))
                } else if name.contains("toDataURL") {
                    Some(("Medium", "Reads canvas data"))
                } else if name.contains("AudioContext") {
                    Some(("Medium", "Uses AudioContext"))
                } else {
                    None
                }
            }
        }
    }
}

impl AstDetector for DangerousApiDetector {
    fn detector_id(&self) -> &str {
        "DET-DANGEROUS-API-002"
    }
    fn detector_name(&self) -> &str {
        "Dangerous & Web API Detector"
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
            if let Some((severity, reason)) = self.check_api(&raw_name) {
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
                    finding_id: format!("F-DANGEROUS-{}", self.findings.len()),
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
