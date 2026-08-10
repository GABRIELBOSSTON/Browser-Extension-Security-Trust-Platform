use super::cache::VtCache;
use super::client::VtClient;
use super::models::VirusTotalReport;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Engine to scan extension files with VirusTotal
pub struct VirusTotalEngine {
    client: Option<VtClient>,
    cache: VtCache,
}

impl VirusTotalEngine {
    /// Initialize the engine.
    /// If `VT_API_KEY` is not set, `client` will be None, and scans will skip API calls gracefully.
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self, String> {
        let cache = VtCache::new(db_path).map_err(|e| format!("Failed to init VT cache: {}", e))?;

        // Setup client conditionally
        #[cfg(not(test))]
        let client = VtClient::new();

        #[cfg(test)]
        let client = Some(VtClient::new_mock());

        Ok(Self { client, cache })
    }

    pub fn is_configured(&self) -> bool {
        self.client.is_some()
    }

    /// Compute SHA256 of a file. Returns hex string.
    pub fn compute_hash(path: &Path) -> Result<String, std::io::Error> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Finds files in the extension directory that should be scanned.
    /// Specifically: manifest.json, background.js, service_worker files, and any .js files.
    fn collect_targets(extension_dir: &Path) -> Vec<PathBuf> {
        let mut targets = Vec::new();

        // 1. Always scan manifest.json
        let manifest = extension_dir.join("manifest.json");
        if manifest.exists() {
            targets.push(manifest);
        }

        // 2. Scan all .js files (including background scripts, service workers, content scripts)
        for entry in WalkDir::new(extension_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "js" {
                        targets.push(entry.path().to_path_buf());
                    }
                }
            }
        }

        targets
    }

    /// Scans the entire extension.
    /// Uses cache when possible. If an API key is missing, only cached responses (if any) are returned.
    pub fn scan_extension(&self, extension_dir: &Path) -> Vec<VirusTotalReport> {
        let targets = Self::collect_targets(extension_dir);
        let mut reports = Vec::new();
        let mut seen_hashes = std::collections::HashSet::new();

        for target in targets {
            if let Ok(hash) = Self::compute_hash(&target) {
                // Avoid scanning the same hash twice in the same pass
                if !seen_hashes.insert(hash.clone()) {
                    continue;
                }

                // Check cache
                if let Some(cached) = self.cache.get_cached(&hash) {
                    reports.push(cached);
                    continue;
                }

                // If not in cache and we have a client, query VT
                if let Some(client) = &self.client {
                    // Small delay to prevent API rate limiting (VT public API is 4 req/min)
                    // In a production app, we'd use a better rate limiter. We just block lightly here.
                    #[cfg(not(test))]
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    match client.get_file_report(&hash) {
                        Ok(Some(report)) => {
                            let _ = self.cache.set_cached(&hash, &report);
                            reports.push(report);
                        }
                        Ok(None) => {
                            // Hash not found on VT - consider creating an "Undetected" report or skipping
                            // We create an empty report to signify we checked and it's clean/unknown.
                            let report = VirusTotalReport::new_empty(&hash);
                            let _ = self.cache.set_cached(&hash, &report);
                            reports.push(report);
                        }
                        Err(e) => {
                            tracing::warn!("VirusTotal API error for hash {}: {}", hash, e);
                        }
                    }
                } else {
                    // No API key configured and no cache entry. Skip.
                }
            }
        }

        reports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_compute_hash() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let mut f = fs::File::create(&path).unwrap();
        write!(f, "hello world").unwrap(); // sha256: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9

        let hash = VirusTotalEngine::compute_hash(&path).unwrap();
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_collect_targets() {
        let dir = tempdir().unwrap();
        fs::File::create(dir.path().join("manifest.json")).unwrap();
        fs::File::create(dir.path().join("background.js")).unwrap();
        fs::File::create(dir.path().join("image.png")).unwrap();

        let targets = VirusTotalEngine::collect_targets(dir.path());
        assert_eq!(targets.len(), 2);

        let file_names: Vec<_> = targets
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(file_names.contains(&"manifest.json"));
        assert!(file_names.contains(&"background.js"));
        assert!(!file_names.contains(&"image.png"));
    }

    #[test]
    fn test_scan_extension_with_mock_client() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("vt.db");
        let engine = VirusTotalEngine::new(&db_path).unwrap();

        // Write a file that will map to a clean hash
        let mut f = fs::File::create(dir.path().join("manifest.json")).unwrap();
        write!(f, "clean_manifest").unwrap();

        let mut f2 = fs::File::create(dir.path().join("bg.js")).unwrap();
        // Since the mock hardcodes "deadbeef" to be malicious, we can't easily force that hash
        // without providing exactly the right bytes. We will just check that it uses the mock and returns 0/60.
        write!(f2, "bg_script").unwrap();

        let reports = engine.scan_extension(dir.path());
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0].malicious, 0);
        assert_eq!(reports[1].malicious, 0);

        // Check if cache was written
        let cache = VtCache::new(&db_path).unwrap();
        let hash1 = VirusTotalEngine::compute_hash(&dir.path().join("manifest.json")).unwrap();
        let cached = cache.get_cached(&hash1);
        assert!(cached.is_some());
    }
}
