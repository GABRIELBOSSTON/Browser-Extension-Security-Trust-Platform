use serde::{Deserialize, Serialize};

/// A single piece of evidence supporting the explanation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// The category of evidence (e.g., "Permission", "AST Finding", "Host Permission")
    pub category: String,
    /// Human-readable description of what was found
    pub detail: String,
    /// The severity level this evidence contributes to
    pub severity: String,
}

/// A concrete recommendation for the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Short action title
    pub action: String,
    /// Detailed description of what to do
    pub description: String,
    /// Priority ordering (1 = most urgent)
    pub priority: u8,
}

/// The complete human-readable security report for one extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityExplanation {
    /// The extension ID this explanation refers to
    pub extension_id: String,
    /// The extension display name
    pub extension_name: String,
    /// Overall risk score (0–100)
    pub risk_score: u32,
    /// Overall risk level label (Safe / Low / Medium / High / Critical)
    pub risk_level: String,
    /// One-paragraph summary of what this extension does and why it is risky
    pub summary: String,
    /// List of concrete evidence items that drove the risk score
    pub evidence: Vec<Evidence>,
    /// Description of realistic threats if this extension were malicious
    pub potential_impact: String,
    /// Ordered list of recommended actions for the user
    pub recommendations: Vec<Recommendation>,
}
