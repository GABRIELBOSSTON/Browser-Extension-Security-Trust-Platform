use crate::application::manifest::mapper::ManifestMapper;
use crate::application::manifest::validator::ManifestValidator;
use crate::domain::entities::Manifest;
use crate::domain::errors::Result;
use crate::infrastructure::manifest::ManifestParser;
use std::path::Path;

pub struct ManifestService;

impl ManifestService {
    pub fn load_manifest(path: &Path) -> Result<Manifest> {
        let raw_manifest = ManifestParser::parse_file(path)?;
        ManifestValidator::validate(&raw_manifest)?;
        ManifestMapper::map(raw_manifest)
    }
}
