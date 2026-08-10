use crate::domain::ast::{Language, ParseResult, ParserConfig};
use crate::domain::errors::Result;

pub trait AstParser: Send + Sync {
    fn parse(
        &self,
        source: &str,
        file_path: &str,
        config: &ParserConfig,
        lang: Language,
    ) -> Result<ParseResult>;
}
