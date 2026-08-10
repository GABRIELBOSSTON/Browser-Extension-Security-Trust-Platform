use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::application::ast_detector::chrome_api_detector::ChromeApiDetector;
use crate::application::ast_detector::dangerous_api_detector::DangerousApiDetector;
use crate::application::ast_detector::data_exfiltration_detector::DataExfiltrationDetector;
use crate::application::ast_detector::fingerprint_detector::FingerprintDetector;
use crate::application::ast_detector::manager::{DetectorManager, DetectorRegistry};
use crate::application::ast_detector::obfuscation_detector::ObfuscationDetector;
use crate::application::ast_detector::rce_detector::RceDetector;
use crate::application::ast_detector::secret_detector::SecretDetector;
use crate::application::ast_walker::{ASTWalker, WalkerConfig};
use crate::infrastructure::ast_walker::swc_walker::SWCAstWalker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ASTFinding {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub reason: String,
    pub node_type: String,
}

pub struct AstScannerService;

impl AstScannerService {
    /// Build the default detector registry used for all file scans.
    /// Sprint 1 Risk Engine v2: 7 detectors covering all threat categories.
    fn build_registry() -> DetectorRegistry {
        let mut registry = DetectorRegistry::new();
        // Original detectors
        registry.add_detector(Box::new(DangerousApiDetector::new()));
        registry.add_detector(Box::new(ChromeApiDetector::new()));
        registry.add_detector(Box::new(SecretDetector::new()));
        // Sprint 1 — Risk Engine v2
        registry.add_detector(Box::new(DataExfiltrationDetector::new()));
        registry.add_detector(Box::new(RceDetector::new()));
        registry.add_detector(Box::new(ObfuscationDetector::new()));
        registry.add_detector(Box::new(FingerprintDetector::new()));
        registry
    }

    pub fn scan_directory(extension_dir: &Path) -> Vec<ASTFinding> {
        let mut all_findings = Vec::new();
        let mut seen_identities = std::collections::HashSet::new();
        let walker = SWCAstWalker::new();
        let config = WalkerConfig::default();

        for entry in WalkDir::new(extension_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "js" || ext == "jsx" {
                        let relative_path = entry
                            .path()
                            .strip_prefix(extension_dir)
                            .unwrap_or(entry.path())
                            .to_string_lossy()
                            .to_string();

                        if let Ok(source) = fs::read_to_string(entry.path()) {
                            let registry = Self::build_registry();
                            let mut manager = DetectorManager::new(registry);

                            if let Ok(result) =
                                walker.walk(&source, &relative_path, &config, &mut manager)
                            {
                                if !result.cancelled {
                                    let detector_results = manager.take_results();

                                    for det_res in detector_results {
                                        for finding in det_res.findings {
                                            // Create stable identity: detector_id + filename + line + column + node_type + reason
                                            // The detector_id isn't in ASTFinding yet, but we have it from det_res
                                            // Actually, different detectors can emit the same reason (overlap).
                                            // To truly dedup, we just use filename + line + column + reason
                                            let identity = format!(
                                                "{}|{}|{}|{}",
                                                relative_path,
                                                finding.location.line,
                                                finding.location.column,
                                                finding.message
                                            );

                                            if !seen_identities.contains(&identity) {
                                                seen_identities.insert(identity);

                                                let severity = finding
                                                    .metadata
                                                    .get("severity")
                                                    .cloned()
                                                    .unwrap_or_else(|| "Low".to_string());
                                                all_findings.push(ASTFinding {
                                                    filename: relative_path.clone(),
                                                    line: finding.location.line,
                                                    column: finding.location.column,
                                                    severity,
                                                    reason: finding.message,
                                                    node_type: format!("{:?}", finding.node_kind),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        all_findings
    }
}
