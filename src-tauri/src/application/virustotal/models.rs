use serde::{Deserialize, Serialize};

/// VirusTotal Scan Report for a specific file hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusTotalReport {
    pub sha256: String,
    pub detection_ratio: String,
    pub malicious: u32,
    pub suspicious: u32,
    pub harmless: u32,
    pub undetected: u32,
    pub timeout: u32,
    pub reputation: i32,
    pub community_score: i32,
    pub first_submission: i64,
    pub last_analysis: i64,
    pub permalink: String,
}

impl VirusTotalReport {
    pub fn new_empty(sha256: &str) -> Self {
        Self {
            sha256: sha256.to_string(),
            detection_ratio: "0/0".to_string(),
            malicious: 0,
            suspicious: 0,
            harmless: 0,
            undetected: 0,
            timeout: 0,
            reputation: 0,
            community_score: 0,
            first_submission: 0,
            last_analysis: 0,
            permalink: format!("https://www.virustotal.com/gui/file/{}", sha256),
        }
    }
}

/// Raw response from VirusTotal v3/files/{hash} API
#[derive(Debug, Deserialize)]
pub struct VtApiResponse {
    pub data: Option<VtData>,
    pub error: Option<VtError>,
}

#[derive(Debug, Deserialize)]
pub struct VtData {
    pub attributes: VtAttributes,
}

#[derive(Debug, Deserialize)]
pub struct VtAttributes {
    pub last_analysis_stats: VtAnalysisStats,
    pub reputation: i32,
    pub first_submission_date: i64,
    pub last_analysis_date: i64,
    pub popular_threat_classification: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct VtAnalysisStats {
    pub malicious: u32,
    pub suspicious: u32,
    pub undetected: u32,
    pub harmless: u32,
    pub timeout: u32,
}

#[derive(Debug, Deserialize)]
pub struct VtError {
    pub message: String,
    pub code: String,
}
