use super::models::{VirusTotalReport, VtApiResponse};
use reqwest::blocking::Client;
use std::env;
use std::time::Duration;

pub struct VtClient {
    api_key: String,
    client: Client,
}

impl VtClient {
    /// Attempts to create a new client using the `VT_API_KEY` env var.
    /// Returns None if the key is not set.
    pub fn new() -> Option<Self> {
        let api_key = env::var("VT_API_KEY").ok()?;
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .ok()?;

        Some(Self { api_key, client })
    }

    /// Queries VirusTotal for a file hash
    pub fn get_file_report(&self, sha256: &str) -> Result<Option<VirusTotalReport>, String> {
        // If testing mode, return mocked response
        #[cfg(test)]
        if self.api_key == "test_key" {
            return Ok(Some(Self::mock_response(sha256)));
        }

        let url = format!("https://www.virustotal.com/api/v3/files/{}", sha256);
        let max_retries = 3;
        let mut attempts = 0;

        while attempts < max_retries {
            attempts += 1;

            let response_result = self
                .client
                .get(&url)
                .header("x-apikey", &self.api_key)
                .send();

            match response_result {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        let vt_resp: VtApiResponse = response
                            .json()
                            .map_err(|e| format!("JSON parsing error: {}", e))?;

                        if let Some(data) = vt_resp.data {
                            let stats = &data.attributes.last_analysis_stats;
                            let total = stats.malicious
                                + stats.suspicious
                                + stats.harmless
                                + stats.undetected
                                + stats.timeout;

                            return Ok(Some(VirusTotalReport {
                                sha256: sha256.to_string(),
                                detection_ratio: format!(
                                    "{}/{}",
                                    stats.malicious + stats.suspicious,
                                    total
                                ),
                                malicious: stats.malicious,
                                suspicious: stats.suspicious,
                                harmless: stats.harmless,
                                undetected: stats.undetected,
                                timeout: stats.timeout,
                                reputation: data.attributes.reputation,
                                community_score: data.attributes.reputation, // Approximate
                                first_submission: data.attributes.first_submission_date,
                                last_analysis: data.attributes.last_analysis_date,
                                permalink: format!(
                                    "https://www.virustotal.com/gui/file/{}",
                                    sha256
                                ),
                            }));
                        }
                        return Ok(None);
                    } else if status.as_u16() == 404 {
                        // File not found in VT database
                        return Ok(None);
                    } else if status.is_server_error() || status.as_u16() == 429 {
                        // Retry on server errors or rate limits
                        if attempts < max_retries {
                            std::thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                        return Err(format!("VirusTotal API error: HTTP {}", status.as_u16()));
                    } else {
                        // Other client errors, do not retry
                        return Err(format!("VirusTotal API error: HTTP {}", status.as_u16()));
                    }
                }
                Err(e) => {
                    if attempts < max_retries {
                        std::thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                    return Err(format!("Network error: {}", e));
                }
            }
        }

        Err("Max retries exceeded".to_string())
    }

    #[cfg(test)]
    pub fn new_mock() -> Self {
        Self {
            api_key: "test_key".to_string(),
            client: Client::new(),
        }
    }

    #[cfg(test)]
    fn mock_response(sha256: &str) -> VirusTotalReport {
        if sha256.starts_with("deadbeef") {
            VirusTotalReport {
                sha256: sha256.to_string(),
                detection_ratio: "5/60".to_string(),
                malicious: 5,
                suspicious: 0,
                harmless: 55,
                undetected: 0,
                timeout: 0,
                reputation: -10,
                community_score: -10,
                first_submission: 1600000000,
                last_analysis: 1600000000,
                permalink: format!("https://www.virustotal.com/gui/file/{}", sha256),
            }
        } else {
            let mut rep = VirusTotalReport::new_empty(sha256);
            rep.detection_ratio = "0/60".to_string();
            rep.harmless = 60;
            rep
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_client_clean_hash() {
        let client = VtClient::new_mock();
        let report = client.get_file_report("abc123clean").unwrap().unwrap();
        assert_eq!(report.malicious, 0);
        assert_eq!(report.detection_ratio, "0/60");
    }

    #[test]
    fn test_mock_client_malicious_hash() {
        let client = VtClient::new_mock();
        let report = client.get_file_report("deadbeef_malware").unwrap().unwrap();
        assert_eq!(report.malicious, 5);
        assert_eq!(report.reputation, -10);
    }
}
