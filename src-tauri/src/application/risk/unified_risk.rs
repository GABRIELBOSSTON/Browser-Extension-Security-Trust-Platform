use crate::application::ast_detector::scanner::{ASTFinding, AstScannerService};
use crate::application::ioc::{IOCFinding, IocEngine};
use crate::application::risk::correlator::RiskCorrelator;
use crate::application::risk::manifest_risk::ManifestRiskEngine;
use crate::application::virustotal::{models::VirusTotalReport, VirusTotalEngine};
use crate::domain::entities::Manifest;
use crate::domain::evidence::{CorrelatedRiskResult, EvidenceItem};
use crate::domain::trust::TrustRegistry;
use std::path::Path;

pub struct UnifiedRiskService;

pub struct UnifiedRiskResult {
    pub correlation: CorrelatedRiskResult,
    pub ast_findings: Vec<ASTFinding>,
    pub ioc_findings: Vec<IOCFinding>,
    pub vt_reports: Vec<VirusTotalReport>,
    pub trusted: bool,
}

impl UnifiedRiskService {
    pub fn analyze_extension(
        extension_dir: &Path,
        extension_id: &str,
        manifest: &Manifest,
        manifest_content: Option<&str>,
        vt_engine: Option<&VirusTotalEngine>,
    ) -> UnifiedRiskResult {
        let mut evidence_items = Vec::new();

        // 1. Manifest Analysis
        let manifest_risk = ManifestRiskEngine::analyze(manifest);
        for finding in &manifest_risk.findings {
            evidence_items.push(finding.clone());
        }

        // 2. AST Analysis
        let ast_findings = if extension_dir.exists() {
            AstScannerService::scan_directory(extension_dir)
        } else {
            Vec::new()
        };

        for finding in &ast_findings {
            let base_score = match finding.severity.as_str() {
                "Critical" => 80,
                "High" => 40,
                "Medium" => 20,
                "Low" => 5,
                _ => 0,
            };
            evidence_items.push(EvidenceItem {
                category: "Code Analysis".to_string(),
                detail: finding.reason.clone(),
                severity: finding.severity.clone(),
                base_score,
            });
        }

        // 3. IOC Analysis
        let mut ioc_findings = if extension_dir.exists() {
            IocEngine::scan_directory(extension_dir)
        } else {
            Vec::new()
        };

        if let Some(content) = manifest_content {
            let manifest_iocs = IocEngine::scan_manifest(content, "manifest.json");
            ioc_findings.extend(manifest_iocs);
        }

        for finding in &ioc_findings {
            let sev_str = format!("{:?}", finding.severity);
            let base_score = match sev_str.as_str() {
                "Critical" => 90,
                "High" => 60,
                "Medium" => 30,
                "Low" => 10,
                _ => 0,
            };
            evidence_items.push(EvidenceItem {
                category: format!("IOC - {}", finding.category),
                detail: format!("Found {}", finding.description),
                severity: sev_str,
                base_score,
            });
        }

        // 4. VirusTotal Analysis
        let vt_reports = if let (Some(vt), true) = (vt_engine, extension_dir.exists()) {
            vt.scan_extension(extension_dir)
        } else {
            Vec::new()
        };

        for vt in &vt_reports {
            if vt.malicious > 0 {
                evidence_items.push(EvidenceItem {
                    category: "VirusTotal".to_string(),
                    detail: "Flagged as Malicious".to_string(),
                    severity: "Critical".to_string(),
                    base_score: 80,
                });
            } else if vt.suspicious > 0 {
                evidence_items.push(EvidenceItem {
                    category: "VirusTotal".to_string(),
                    detail: "Flagged as Suspicious".to_string(),
                    severity: "High".to_string(),
                    base_score: 40,
                });
            }
        }

        // 5. Trust Registry
        let trusted = TrustRegistry::is_trusted(extension_id);
        if trusted {
            evidence_items.push(EvidenceItem {
                category: "Trust".to_string(),
                detail: "Trusted Publisher".to_string(),
                severity: "Good".to_string(),
                base_score: -30,
            });
        }

        // 6. Correlate
        let correlation = RiskCorrelator::correlate(evidence_items);

        UnifiedRiskResult {
            correlation,
            ast_findings,
            ioc_findings,
            vt_reports,
            trusted,
        }
    }
}
