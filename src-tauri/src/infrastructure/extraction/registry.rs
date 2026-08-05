use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use crate::domain::extraction::{ArchiveExtractor, ArchiveExtractorRegistry};
use crate::domain::errors::{DomainError, Result};

pub struct ExtractorRegistryImpl {
    extractors: Vec<Arc<dyn ArchiveExtractor>>,
}

impl ExtractorRegistryImpl {
    pub fn new(extractors: Vec<Arc<dyn ArchiveExtractor>>) -> Self {
        Self { extractors }
    }
}

impl ArchiveExtractorRegistry for ExtractorRegistryImpl {
    fn get_extractor(&self, source_archive: &Path) -> Result<Arc<dyn ArchiveExtractor>> {
        let mut file = File::open(source_archive)
            .map_err(|e| DomainError::IoError(format!("Failed to open file for registry: {}", e)))?;
            
        let mut header = [0u8; 8];
        let bytes_read = file.read(&mut header)
            .map_err(|_| DomainError::CorruptedArchive)?;
            
        let header_slice = &header[..bytes_read];

        for extractor in &self.extractors {
            if extractor.supports(header_slice) {
                return Ok(extractor.clone());
            }
        }
        
        Err(DomainError::UnsupportedArchive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    struct DummyZipExtractor;
    impl ArchiveExtractor for DummyZipExtractor {
        fn extract(&self, _src: &Path, _dest: &crate::domain::extraction::SandboxContext) -> Result<usize> { Ok(0) }
        fn supports(&self, header: &[u8]) -> bool { header.starts_with(b"PK\x03\x04") }
    }

    struct DummyCrxExtractor;
    impl ArchiveExtractor for DummyCrxExtractor {
        fn extract(&self, _src: &Path, _dest: &crate::domain::extraction::SandboxContext) -> Result<usize> { Ok(0) }
        fn supports(&self, header: &[u8]) -> bool { header.starts_with(b"Cr24") }
    }

    #[test]
    fn test_registry_routing() {
        let mut zip_file = NamedTempFile::new().unwrap();
        zip_file.write_all(b"PK\x03\x04_content").unwrap();
        
        let mut crx_file = NamedTempFile::new().unwrap();
        crx_file.write_all(b"Cr24_content").unwrap();
        
        let registry = ExtractorRegistryImpl::new(vec![
            Arc::new(DummyZipExtractor),
            Arc::new(DummyCrxExtractor),
        ]);
        
        // Extension doesn't matter, purely magic byte driven
        let ext1 = registry.get_extractor(zip_file.path());
        assert!(ext1.is_ok()); // Matched PK

        let ext2 = registry.get_extractor(crx_file.path());
        assert!(ext2.is_ok()); // Matched Cr24
    }
}
