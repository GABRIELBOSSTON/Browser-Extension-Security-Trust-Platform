use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::domain::errors::{DomainError, Result};
use crate::infrastructure::manifest::models::RawManifest;

const MAX_MANIFEST_SIZE_BYTES: u64 = 10 * 1024 * 1024; // 10 MB

pub struct ManifestParser;

impl ManifestParser {
    pub fn parse_file(path: &Path) -> Result<RawManifest> {
        if !path.exists() || !path.is_file() {
            return Err(DomainError::ManifestFileNotFound(path.to_string_lossy().to_string()));
        }

        let metadata = std::fs::metadata(path)
            .map_err(|e| DomainError::IoError(format!("Failed to read metadata: {}", e)))?;

        if metadata.len() > MAX_MANIFEST_SIZE_BYTES {
            return Err(DomainError::ManifestFileTooLarge(path.to_string_lossy().to_string()));
        }

        let mut file = File::open(path)
            .map_err(|e| DomainError::IoError(format!("Failed to open file: {}", e)))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| DomainError::IoError(format!("Failed to read string (invalid UTF-8?): {}", e)))?;

        // Deserialize ignoring unknown fields
        let raw: RawManifest = serde_json::from_str(&content)
            .map_err(|e| DomainError::InvalidManifestJson(format!("{}", e)))?;

        Ok(raw)
    }
}
