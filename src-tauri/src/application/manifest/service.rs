use std::path::Path;
use crate::domain::errors::Result;
use crate::domain::entities::Manifest;
use crate::infrastructure::manifest::ManifestParser;
use crate::application::manifest::validator::ManifestValidator;
use crate::application::manifest::mapper::ManifestMapper;

pub struct ManifestService;

impl ManifestService {
    pub fn load_manifest(path: &Path) -> Result<Manifest> {
        let raw_manifest = ManifestParser::parse_file(path)?;
        ManifestValidator::validate(&raw_manifest)?;
        ManifestMapper::map(raw_manifest)
    }
}
