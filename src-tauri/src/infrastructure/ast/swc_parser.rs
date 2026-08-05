use std::sync::Arc;
use std::time::Instant;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig, EsConfig};

use crate::application::ast::parser::AstParser;
use crate::domain::ast::{
    ASTDocument, Language, ParseDiagnostic, ParseResult, ParseStatistics, ParserConfig,
};
use crate::domain::errors::Result;

pub struct SWCAstParser;

impl SWCAstParser {
    pub fn new() -> Self {
        Self
    }
}

impl AstParser for SWCAstParser {
    fn parse(&self, source: &str, file_path: &str, config: &ParserConfig, lang: Language) -> Result<ParseResult> {
        let start = Instant::now();
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom(file_path.into()), source.into());

        let syntax = match lang {
            Language::TypeScript | Language::TSX => Syntax::Typescript(TsConfig {
                tsx: matches!(lang, Language::TSX),
                decorators: config.allow_decorators,
                ..Default::default()
            }),
            Language::JavaScript | Language::JSX => Syntax::Es(EsConfig {
                jsx: matches!(lang, Language::JSX),
                decorators: config.allow_decorators,
                ..Default::default()
            }),
        };

        let lexer = Lexer::new(
            syntax,
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        
        let mut diagnostics = Vec::new();
        let mut ast_document = None;
        let mut root_node_count = 0;

        match parser.parse_module() {
            Ok(module) => {
                root_node_count = module.body.len();
                ast_document = Some(ASTDocument {
                    root_node_count,
                });
            }
            Err(err) => {
                let span = err.span();
                let loc = cm.lookup_char_pos(span.lo);
                diagnostics.push(ParseDiagnostic {
                    severity: "Error".to_string(),
                    message: err.into_kind().msg().to_string(),
                    line: loc.line,
                    column: loc.col_display,
                    span: (span.lo.0 as usize, span.hi.0 as usize),
                });
            }
        }

        // SWC parser might also have non-fatal errors
        for err in parser.take_errors() {
            let span = err.span();
            let loc = cm.lookup_char_pos(span.lo);
            diagnostics.push(ParseDiagnostic {
                severity: "Warning".to_string(),
                message: err.into_kind().msg().to_string(),
                line: loc.line,
                column: loc.col_display,
                span: (span.lo.0 as usize, span.hi.0 as usize),
            });
        }

        let duration = start.elapsed();
        let statistics = ParseStatistics {
            lines: source.lines().count(),
            characters: source.len(),
            parse_duration_ms: duration.as_millis() as u64,
        };

        Ok(ParseResult {
            source_file: file_path.to_string(),
            language: lang,
            ast_document,
            diagnostics,
            statistics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast::{Language, ParserConfig};

    #[test]
    fn test_parse_valid_js() {
        let parser = SWCAstParser::new();
        let source = "const x = 10; function test() { return x; }";
        let config = ParserConfig::default();
        let result = parser.parse(source, "test.js", &config, Language::JavaScript).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert!(result.ast_document.is_some());
        assert_eq!(result.ast_document.unwrap().root_node_count, 2);
    }

    #[test]
    fn test_parse_invalid_js() {
        let parser = SWCAstParser::new();
        let source = "function test() { return x"; // missing closing brace
        let config = ParserConfig::default();
        let result = parser.parse(source, "test.js", &config, Language::JavaScript).unwrap();
        
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.diagnostics[0].severity, "Error");
    }

    #[test]
    fn test_parse_valid_ts() {
        let parser = SWCAstParser::new();
        let source = "let x: number = 10; interface User { name: string; }";
        let config = ParserConfig::default();
        let result = parser.parse(source, "test.ts", &config, Language::TypeScript).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert!(result.ast_document.is_some());
        assert_eq!(result.ast_document.unwrap().root_node_count, 2);
    }

    #[test]
    fn test_parse_jsx() {
        let parser = SWCAstParser::new();
        let source = "const el = <div>Hello</div>;";
        let config = ParserConfig::default();
        let result = parser.parse(source, "test.jsx", &config, Language::JSX).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert!(result.ast_document.is_some());
    }

    #[test]
    fn test_parse_empty() {
        let parser = SWCAstParser::new();
        let source = "";
        let config = ParserConfig::default();
        let result = parser.parse(source, "empty.js", &config, Language::JavaScript).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.ast_document.unwrap().root_node_count, 0);
    }

    #[test]
    fn test_parse_minified() {
        let parser = SWCAstParser::new();
        let source = "function a(b,c){return b+c}var d=a(1,2);console.log(d);";
        let config = ParserConfig::default();
        let result = parser.parse(source, "minified.js", &config, Language::JavaScript).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.ast_document.unwrap().root_node_count, 3);
    }

    #[test]
    fn test_parse_obfuscated() {
        let parser = SWCAstParser::new();
        let source = "var _0x1234=['log','Hello\\x20World'];console[_0x1234[0]](_0x1234[1]);";
        let config = ParserConfig::default();
        let result = parser.parse(source, "obfuscated.js", &config, Language::JavaScript).unwrap();
        
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.ast_document.unwrap().root_node_count, 2);
    }
}
