use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    TypeScript,
    JSX,
    TSX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseDiagnostic {
    pub severity: String, // e.g., "Error", "Warning"
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub span: (usize, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseStatistics {
    pub lines: usize,
    pub characters: usize,
    pub parse_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASTDocument {
    // Abstract representation of the AST nodes.
    // For this foundation, it is intentionally minimal.
    // In future iterations, we will map standard SWC/OXC nodes into custom domain representations here.
    pub root_node_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub source_file: String,
    pub language: Language,
    pub ast_document: Option<ASTDocument>,
    pub diagnostics: Vec<ParseDiagnostic>,
    pub statistics: ParseStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub allow_typescript: bool,
    pub allow_jsx: bool,
    pub allow_decorators: bool,
    pub strict_mode: bool,
    pub capture_comments: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            allow_typescript: true,
            allow_jsx: true,
            allow_decorators: true,
            strict_mode: false,
            capture_comments: false,
        }
    }
}
