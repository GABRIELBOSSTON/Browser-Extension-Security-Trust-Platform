use thiserror::Error;

/// Core Domain Error types for AEP
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid Risk Score value: {0}. Score must be between 0.0 and 100.0")]
    InvalidRiskScore(f64),

    #[error("Database connection error: {0}")]
    DatabaseConnection(String),

    #[error("Database constraint violation: {0}")]
    DatabaseConstraint(String),

    #[error("Database query error: {0}")]
    DatabaseQuery(String),

    #[error("Database serialization error: {0}")]
    DatabaseSerialization(String),

    #[error("Unsupported archive format")]
    UnsupportedArchive,

    #[error("Corrupted archive")]
    CorruptedArchive,

    #[error("Zip-Slip vulnerability detected: {0}")]
    ZipSlipDetected(String),

    #[error("Failed to create sandbox")]
    SandboxCreationFailed,

    #[error("Failed to clean up sandbox")]
    SandboxCleanupFailed,

    #[error("Validation failed: {0:?}")]
    ValidationFailed(Vec<String>),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Manifest file not found: {0}")]
    ManifestFileNotFound(String),

    #[error("Invalid JSON format in manifest: {0}")]
    InvalidManifestJson(String),

    #[error("Unsupported manifest version: {0}")]
    UnsupportedManifestVersion(u32),

    #[error("Missing required field in manifest: {0}")]
    MissingRequiredField(String),

    #[error("Manifest file too large: {0}")]
    ManifestFileTooLarge(String),
}

pub type Result<T> = std::result::Result<T, DomainError>;
