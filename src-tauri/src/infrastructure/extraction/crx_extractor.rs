use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use crate::domain::extraction::{ArchiveExtractor, SandboxContext};
use crate::domain::errors::{DomainError, Result};
use super::zip_extractor::ZipExtractor;

pub struct CrxExtractor {
    zip_extractor: ZipExtractor,
}

impl CrxExtractor {
    pub fn new() -> Self {
        Self {
            zip_extractor: ZipExtractor::new(),
        }
    }
}

impl ArchiveExtractor for CrxExtractor {
    fn extract(&self, source_archive: &Path, destination_sandbox: &SandboxContext) -> Result<usize> {
        let mut file = File::open(source_archive)
            .map_err(|e| DomainError::IoError(e.to_string()))?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).map_err(|_| DomainError::CorruptedArchive)?;
        
        if &magic != b"Cr24" {
            return Err(DomainError::UnsupportedArchive);
        }

        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes).map_err(|_| DomainError::CorruptedArchive)?;
        let version = u32::from_le_bytes(version_bytes);

        let mut header_size = 0;

        if version == 2 {
            let mut pubkey_len_bytes = [0u8; 4];
            let mut sig_len_bytes = [0u8; 4];
            file.read_exact(&mut pubkey_len_bytes).map_err(|_| DomainError::CorruptedArchive)?;
            file.read_exact(&mut sig_len_bytes).map_err(|_| DomainError::CorruptedArchive)?;
            let pubkey_len = u32::from_le_bytes(pubkey_len_bytes);
            let sig_len = u32::from_le_bytes(sig_len_bytes);
            header_size = 16 + pubkey_len + sig_len;
        } else if version == 3 {
            let mut header_size_bytes = [0u8; 4];
            file.read_exact(&mut header_size_bytes).map_err(|_| DomainError::CorruptedArchive)?;
            header_size = 12 + u32::from_le_bytes(header_size_bytes);
        } else {
            return Err(DomainError::UnsupportedArchive);
        }

        // Slice the zip payload into a temporary file or pass it to zip_extractor
        file.seek(SeekFrom::Start(header_size as u64))
            .map_err(|_| DomainError::CorruptedArchive)?;

        // Instead of writing a tmp file, we can create a temp zip file inside the sandbox
        let temp_zip_path = destination_sandbox.root_path.join(".temp_payload.zip");
        let mut temp_zip = File::create(&temp_zip_path)
            .map_err(|e| DomainError::IoError(e.to_string()))?;
            
        std::io::copy(&mut file, &mut temp_zip)
            .map_err(|e| DomainError::IoError(e.to_string()))?;

        // Delegate to ZipExtractor
        let res = self.zip_extractor.extract(&temp_zip_path, destination_sandbox);

        // Cleanup the temp zip
        let _ = std::fs::remove_file(temp_zip_path);

        res
    }

    fn supports(&self, header_bytes: &[u8]) -> bool {
        // CRX magic bytes: Cr24 (43 72 32 34)
        header_bytes.len() >= 4 && &header_bytes[0..4] == b"Cr24"
    }
}
