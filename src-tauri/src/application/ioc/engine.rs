use super::detector::{
    CryptoIocDetector, EncodedPayloadIocDetector, IocDetector, NetworkIocDetector,
    ObfuscationIocDetector, SecretIocDetector, WebAssemblyIocDetector,
};
use super::models::IOCFinding;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Central IOC Engine — runs all detectors over a file or directory.
pub struct IocEngine;

impl IocEngine {
    /// Build the registry of all detectors.
    fn build_detectors() -> Vec<Box<dyn IocDetector>> {
        vec![
            Box::new(NetworkIocDetector),
            Box::new(SecretIocDetector),
            Box::new(ObfuscationIocDetector),
            Box::new(CryptoIocDetector),
            Box::new(WebAssemblyIocDetector),
            Box::new(EncodedPayloadIocDetector),
        ]
    }

    /// Scan all `.js` / `.jsx` files in an extension directory.
    pub fn scan_directory(extension_dir: &Path) -> Vec<IOCFinding> {
        let detectors = Self::build_detectors();
        let mut all_findings: Vec<IOCFinding> = Vec::new();
        let mut finding_counter: usize = 0;

        for entry in WalkDir::new(extension_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.path().is_file() {
                continue;
            }
            let is_js = entry
                .path()
                .extension()
                .map(|e| e == "js" || e == "jsx")
                .unwrap_or(false);
            if !is_js {
                continue;
            }

            let relative = entry
                .path()
                .strip_prefix(extension_dir)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();

            if let Ok(source) = fs::read_to_string(entry.path()) {
                for detector in &detectors {
                    let mut findings = detector.scan(&source, &relative);
                    // Re-sequence IDs so they are globally unique across files
                    for f in &mut findings {
                        finding_counter += 1;
                        f.id = format!("IOC-{:04}", finding_counter);
                    }
                    all_findings.extend(findings);
                }
            }
        }

        let mut seen = std::collections::HashSet::new();
        let mut deduped = Vec::new();
        for finding in all_findings {
            let id = format!("{}|{}|{}", finding.file, finding.line, finding.description);
            if seen.insert(id) {
                deduped.push(finding);
            }
        }
        all_findings = deduped;

        // Sort: Critical first, then High, Medium, Low; secondary: file + line
        all_findings.sort_by(|a, b| {
            Self::severity_order(&a.severity.to_string())
                .cmp(&Self::severity_order(&b.severity.to_string()))
                .then(a.file.cmp(&b.file))
                .then(a.line.cmp(&b.line))
        });

        all_findings
    }

    /// Scan raw manifest JSON text for IOCs (Network + Secret patterns).
    pub fn scan_manifest(manifest_json: &str, filename: &str) -> Vec<IOCFinding> {
        let detectors: Vec<Box<dyn IocDetector>> =
            vec![Box::new(NetworkIocDetector), Box::new(SecretIocDetector)];

        let mut findings: Vec<IOCFinding> = Vec::new();
        for detector in &detectors {
            findings.extend(detector.scan(manifest_json, filename));
        }
        findings
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_scan_directory_detects_discord_webhook() {
        let dir = tempdir().unwrap();
        let js_path = dir.path().join("bg.js");
        let mut f = fs::File::create(&js_path).unwrap();
        writeln!(f, r#"fetch("https://discord.com/api/webhooks/123/abc");"#).unwrap();

        let findings = IocEngine::scan_directory(dir.path());
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("Discord"));
    }

    #[test]
    fn test_scan_directory_skips_non_js() {
        let dir = tempdir().unwrap();
        let txt_path = dir.path().join("readme.txt");
        let mut f = fs::File::create(&txt_path).unwrap();
        writeln!(f, "discord.com/api/webhooks — just a text file").unwrap();

        let findings = IocEngine::scan_directory(dir.path());
        assert!(findings.is_empty(), "Non-JS files must be skipped");
    }

    #[test]
    fn test_scan_directory_multiple_categories() {
        let dir = tempdir().unwrap();
        let js_path = dir.path().join("evil.js");
        let mut f = fs::File::create(&js_path).unwrap();
        writeln!(
            f,
            r#"
const key = "{}{}";
fetch("https://discord.com/api/webhooks/123/abc");
const code = atob("aGVsbG8gd29ybGQ=");
"#,
            "AIzaSy", "D-9tSrke72I6e0IV6zL73XXXXXXXXXXXX"
        )
        .unwrap();

        let findings = IocEngine::scan_directory(dir.path());
        let categories: std::collections::HashSet<_> =
            findings.iter().map(|f| f.category.to_string()).collect();
        assert!(categories.contains("Network"));
        assert!(categories.contains("Secret"));
        assert!(categories.contains("Obfuscation"));
    }

    #[test]
    fn test_scan_directory_ids_are_unique() {
        let dir = tempdir().unwrap();
        let js_path = dir.path().join("multi.js");
        let mut f = fs::File::create(&js_path).unwrap();
        writeln!(
            f,
            "fetch(\"https://discord.com/api/webhooks/1\");\nfetch(\"https://api.telegram.org/bot123\");"
        )
        .unwrap();

        let findings = IocEngine::scan_directory(dir.path());
        let ids: Vec<_> = findings.iter().map(|f| &f.id).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(ids.len(), unique.len(), "All finding IDs must be unique");
    }

    #[test]
    fn test_scan_manifest_detects_network_ioc() {
        let manifest = r#"{ "update_url": "https://pastebin.com/raw/abc" }"#;
        let findings = IocEngine::scan_manifest(manifest, "manifest.json");
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("Pastebin"));
    }

    #[test]
    fn test_scan_directory_findings_sorted_critical_first() {
        let dir = tempdir().unwrap();
        let js_path = dir.path().join("mixed.js");
        let mut f = fs::File::create(&js_path).unwrap();
        writeln!(
            f,
            "const h = md5(\"test\");\nfetch(\"https://discord.com/api/webhooks/x\");\n"
        )
        .unwrap();

        let findings = IocEngine::scan_directory(dir.path());
        if findings.len() >= 2 {
            let first_sev = IocEngine::severity_order(&findings[0].severity.to_string());
            let last_sev =
                IocEngine::severity_order(&findings[findings.len() - 1].severity.to_string());
            assert!(first_sev <= last_sev, "Should be sorted Critical first");
        }
    }
}
