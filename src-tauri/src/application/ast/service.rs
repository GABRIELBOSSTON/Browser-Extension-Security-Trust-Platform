use std::path::Path;
use crate::domain::ast::{Language, ParseResult, ParserConfig};
use crate::domain::errors::{DomainError, Result};
use super::parser::AstParser;
use super::factory::{ParserFactory, ParserBackend};

pub struct ASTService {
    parser: Box<dyn AstParser>,
}

impl ASTService {
    pub fn new(backend: ParserBackend) -> Self {
        Self {
            parser: ParserFactory::create_parser(backend),
        }
    }

    pub fn parse_file(&self, file_path: &Path, config: &ParserConfig, lang: Language) -> Result<ParseResult> {
        let source = std::fs::read_to_string(file_path).map_err(|e| DomainError::IoError(e.to_string()))?;
        self.parser.parse(&source, file_path.to_string_lossy().as_ref(), config, lang)
    }

    pub fn parse_source(&self, source: &str, file_name: &str, config: &ParserConfig, lang: Language) -> Result<ParseResult> {
        self.parser.parse(source, file_name, config, lang)
    }
}
