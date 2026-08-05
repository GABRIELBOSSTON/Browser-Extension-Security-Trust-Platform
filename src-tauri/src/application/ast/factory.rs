use super::parser::AstParser;
use crate::infrastructure::ast::swc_parser::SWCAstParser;

pub enum ParserBackend {
    Swc,
    // Future: Oxc, TreeSitter
}

pub struct ParserFactory;

impl ParserFactory {
    pub fn create_parser(backend: ParserBackend) -> Box<dyn AstParser> {
        match backend {
            ParserBackend::Swc => Box::new(SWCAstParser::new()),
        }
    }
}
