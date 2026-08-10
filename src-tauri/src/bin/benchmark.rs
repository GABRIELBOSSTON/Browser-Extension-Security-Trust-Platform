use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use antigraviiti_extension_protect::application::risk::unified_risk::UnifiedRiskService;
use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize)]
struct BenchmarkResult {
    name: String,
    expected_risk_level: String,
    actual_risk_level: String,
    expected_risk_score_range: (u32, u32),
    actual_risk_score: u32,
    ast_findings_count: usize,
    ioc_findings_count: usize,
    vt_reports_count: usize,
    evidence_items: Vec<String>,
}

#[tokio::main]
async fn main() {
    println!("Starting Detection Accuracy Benchmark...\n");

    let benchmark_dir = PathBuf::from("../tests/benchmark_data");
    if !benchmark_dir.exists() {
        eprintln!("Benchmark data directory not found!");
        return;
    }

    let mut results = Vec::new();

    for entry in fs::read_dir(benchmark_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let category_name = path.file_name().unwrap().to_str().unwrap().to_string();

            let expected_level = match category_name.as_str() {
                "benign" => "Low",
                "suspicious" => "High", // Or Medium depending on correlator output
                "malicious_synthetic" => "Critical",
                _ => "Unknown",
            };

            let expected_score_range = match category_name.as_str() {
                "benign" => (0, 20),
                "suspicious" => (40, 75), // Not quite critical
                "malicious_synthetic" => (80, 100),
                _ => (0, 100),
            };

            let manifest_path = path.join("manifest.json");
            if !manifest_path.exists() {
                continue;
            }

            let manifest = ManifestService::load_manifest(&manifest_path).unwrap();
            let manifest_content = fs::read_to_string(&manifest_path).unwrap();

            let risk_result = UnifiedRiskService::analyze_extension(
                &path,
                "benchmark_test_ext",
                &manifest,
                Some(&manifest_content),
                None,
            );

            let final_score = risk_result.correlation.final_score;
            let final_level = risk_result.correlation.final_level;

            let evidence_strings: Vec<String> = risk_result
                .correlation
                .evidence
                .iter()
                .map(|e| format!("[{}] {}: {}", e.severity, e.category, e.detail))
                .collect();

            results.push(BenchmarkResult {
                name: category_name,
                expected_risk_level: expected_level.to_string(),
                actual_risk_level: final_level,
                expected_risk_score_range: expected_score_range,
                actual_risk_score: final_score,
                ast_findings_count: risk_result.ast_findings.len(),
                ioc_findings_count: risk_result.ioc_findings.len(),
                vt_reports_count: 0,
                evidence_items: evidence_strings,
            });
        }
    }

    let json_output = serde_json::to_string_pretty(&results).unwrap();
    println!("{}", json_output);
    fs::write("benchmark_output.json", json_output).unwrap();
    println!("\nBenchmark complete! Results written to benchmark_output.json");
}
