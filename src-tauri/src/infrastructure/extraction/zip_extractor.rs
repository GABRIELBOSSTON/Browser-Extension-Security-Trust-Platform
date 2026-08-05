use std::fs;
use std::io;
use std::path::{Path, PathBuf, Component};
use std::fs::File;
use zip::ZipArchive;
use crate::domain::extraction::{ArchiveExtractor, SandboxContext};
use crate::domain::errors::{DomainError, Result};

pub struct ZipExtractor;

impl ZipExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Safely normalizes a path in memory resolving `.` and `..` without hitting disk
    fn normalize_path(path: &Path) -> PathBuf {
        let mut normalized = PathBuf::new();
        for component in path.components() {
            match component {
                Component::ParentDir => { normalized.pop(); },
                Component::Normal(c) => normalized.push(c),
                Component::RootDir => { normalized.push("/"); },
                _ => {}
            }
        }
        normalized
    }
}

impl ArchiveExtractor for ZipExtractor {
    fn extract(&self, source_archive: &Path, destination_sandbox: &SandboxContext) -> Result<usize> {
        let file = File::open(source_archive)
            .map_err(|e| DomainError::IoError(format!("Failed to open archive: {}", e)))?;
            
        let mut archive = ZipArchive::new(file)
            .map_err(|_| DomainError::CorruptedArchive)?;

        let mut extracted_count = 0;
        let sandbox_root = &destination_sandbox.root_path;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|_| DomainError::CorruptedArchive)?;

            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue, // Skip potentially malicious entry that enclosed_name outright rejects
            };

            let mut target_path = sandbox_root.clone();
            target_path.push(outpath);
            
            let normalized_target = Self::normalize_path(&target_path);
            let normalized_sandbox = Self::normalize_path(sandbox_root);

            if !normalized_target.starts_with(&normalized_sandbox) {
                return Err(DomainError::ZipSlipDetected(
                    format!("Path traversal attempt detected: {:?}", target_path)
                ));
            }

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&target_path)
                    .map_err(|e| DomainError::IoError(e.to_string()))?;
            } else {
                if let Some(p) = target_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)
                            .map_err(|e| DomainError::IoError(e.to_string()))?;
                    }
                }
                let mut outfile = File::create(&target_path)
                    .map_err(|e| DomainError::IoError(e.to_string()))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|e| DomainError::IoError(e.to_string()))?;
                
                extracted_count += 1;
            }
        }

        Ok(extracted_count)
    }

    fn supports(&self, header_bytes: &[u8]) -> bool {
        // ZIP magic bytes: PK\x03\x04 (50 4B 03 04)
        header_bytes.len() >= 4 && &header_bytes[0..4] == b"PK\x03\x04"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_normalize_path_basic() {
        let path = PathBuf::from("/tmp/sandbox/ext/file.js");
        let norm = ZipExtractor::normalize_path(&path);
        assert_eq!(norm, PathBuf::from("/tmp/sandbox/ext/file.js"));
    }

    #[test]
    fn test_normalize_path_traversal() {
        let path = PathBuf::from("/tmp/sandbox/ext/../../etc/passwd");
        let norm = ZipExtractor::normalize_path(&path);
        assert_eq!(norm, PathBuf::from("/tmp/etc/passwd"));
    }

    #[test]
    fn test_zip_slip_defense_logic() {
        let sandbox_root = PathBuf::from("/tmp/sandbox_123");
        
        let safe_target = sandbox_root.join("script.js");
        let safe_norm = ZipExtractor::normalize_path(&safe_target);
        assert!(safe_norm.starts_with(ZipExtractor::normalize_path(&sandbox_root)));
        
        let malicious_target = sandbox_root.join("../../etc/passwd");
        let malicious_norm = ZipExtractor::normalize_path(&malicious_target);
        assert!(!malicious_norm.starts_with(ZipExtractor::normalize_path(&sandbox_root)));
    }
}
