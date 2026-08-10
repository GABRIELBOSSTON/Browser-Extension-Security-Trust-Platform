use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstNodeKind {
    Module,
    FunctionDeclaration,
    VariableDeclaration,
    CallExpression,
    MemberExpression,
    Identifier,
    ImportDeclaration,
    ExportDeclaration,
    ClassDeclaration,
    ArrowFunction,
    NewExpression,
    ImportExpression,
    StringLiteral,
    TemplateLiteral,
}

#[derive(Debug, Clone)]
pub enum ASTNodeEvent {
    Enter(AstNodeKind),
    Leave(AstNodeKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraversalLifecycleEvent {
    TraversalStarted,
    TraversalFinished,
    TraversalCancelled,
}

#[derive(Debug, Clone, Default)]
pub struct VisitorContext {
    pub current_depth: usize,
    pub visited_count: usize,
    pub current_file: Option<String>,
    pub current_function: Option<String>,
    pub current_class: Option<String>,
    pub current_module: Option<String>,
    pub parent_stack: Vec<AstNodeKind>,
    pub scope_depth: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct TraversalResult {
    pub visited_node_count: usize,
    pub traversal_duration_ms: u64,
    pub diagnostics: Vec<String>,
    pub warnings: Vec<String>,
    pub per_node_statistics: HashMap<String, usize>,
    pub max_depth: usize,
    pub cancelled: bool,
}

pub trait ASTVisitor: Send + Sync {
    fn enter(&mut self, node: AstNodeKind, context: &mut VisitorContext);
    fn leave(&mut self, node: AstNodeKind, context: &mut VisitorContext);
    fn lifecycle(&mut self, _event: TraversalLifecycleEvent, _context: &mut VisitorContext) {}
}
