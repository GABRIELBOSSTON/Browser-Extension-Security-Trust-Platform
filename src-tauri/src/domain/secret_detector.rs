use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::domain::ast_detector::SourceLocation;
use crate::domain::call_graph::FunctionId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretId(pub u32); // Stable deterministic identifier representing the secret type

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretSourceKind {
    StringLiteral,
    TemplateLiteral,
    EnvironmentVariable,
    Concatenation,
    Unknown,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretCategory {
    CloudProvider,
    PaymentProvider,
    AIProvider,
    CommunicationProvider,
    VersionControl,
    Authentication,
    Unknown,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretType {
    GoogleApiKey,
    AwsAccessKey,
    AwsSecretKey,
    Jwt,
    FirebaseKey,
    StripeKey,
    DiscordToken,
    SlackToken,
    GithubToken,
    OpenAiKey,
    AnthropicKey,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMatch {
    pub secret_id: SecretId,
    pub secret_type: SecretType,
    pub category: SecretCategory,
    pub confidence: MatchConfidence,
    pub source_kind: SecretSourceKind,
    pub preview: String, // Truncated/masked for security logging safely (max 64 chars)
    pub source_location: SourceLocation,
    pub function_id: FunctionId,
    pub call_depth: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretInventory {
    pub matches: Vec<SecretMatch>,
    pub unique_secret_ids: HashSet<SecretId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretStatistics {
    pub total_matches: usize,
    pub unique_matches: usize,
    pub unknown_matches: usize,
    pub matches_by_type: HashMap<SecretType, usize>, // Scalable map
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretResult {
    pub detector_id: String,
    pub detector_version: String,
    pub inventory: SecretInventory,
    pub statistics: SecretStatistics,
    pub elapsed_ms: u64,
    pub diagnostics: Vec<String>,
}
