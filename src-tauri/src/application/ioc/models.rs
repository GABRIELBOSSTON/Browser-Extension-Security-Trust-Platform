use serde::{Deserialize, Serialize};

/// Categorizes the type of IOC detected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum IocCategory {
    Network,
    Secret,
    Obfuscation,
    Crypto,
    WebAssembly,
    EncodedPayload,
}

impl std::fmt::Display for IocCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IocCategory::Network => write!(f, "Network"),
            IocCategory::Secret => write!(f, "Secret"),
            IocCategory::Obfuscation => write!(f, "Obfuscation"),
            IocCategory::Crypto => write!(f, "Crypto"),
            IocCategory::WebAssembly => write!(f, "WebAssembly"),
            IocCategory::EncodedPayload => write!(f, "EncodedPayload"),
        }
    }
}

/// Severity level of the detected IOC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IocSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for IocSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IocSeverity::Critical => write!(f, "Critical"),
            IocSeverity::High => write!(f, "High"),
            IocSeverity::Medium => write!(f, "Medium"),
            IocSeverity::Low => write!(f, "Low"),
        }
    }
}

/// A single Indicator of Compromise found during scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOCFinding {
    /// Unique finding ID (e.g. "IOC-NET-001")
    pub id: String,
    /// Category of this IOC
    pub category: IocCategory,
    /// Severity level
    pub severity: IocSeverity,
    /// Short human-readable title
    pub title: String,
    /// Detailed description explaining why this is suspicious
    pub description: String,
    /// The exact string or pattern that triggered this finding
    pub matched_pattern: String,
    /// Relative file path where the IOC was found
    pub file: String,
    /// 1-indexed line number
    pub line: usize,
    /// 0-indexed column number
    pub column: usize,
}
