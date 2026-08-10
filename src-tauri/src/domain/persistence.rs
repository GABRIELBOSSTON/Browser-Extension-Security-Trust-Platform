use crate::domain::errors::DomainError;
use crate::domain::risk_calculator::RiskAssessment;
use crate::domain::types::{RiskScore, Severity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScanId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub risk_assessment: RiskAssessment,
    pub findings_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRecord {
    pub id: ScanId,
    pub extension_id: String,
    pub extension_name: String,
    pub version: String,
    pub risk_score: RiskScore,
    pub severity: Severity,
    pub scan_timestamp_utc: DateTime<Utc>,
    pub report: ScanReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditAction {
    ScanCompleted,
    ScanFailed,
    ExtensionDiscovered,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub log_id: Uuid,
    pub action: AuditAction,
    pub metadata: AuditMetadata,
    pub timestamp_utc: DateTime<Utc>,
}

pub trait ScanRepository: Send + Sync {
    fn save_scan(&self, record: &ScanRecord) -> Result<(), DomainError>;
    fn get_scan(&self, id: &ScanId) -> Result<Option<ScanRecord>, DomainError>;
    fn get_recent_scans(&self, limit: usize) -> Result<Vec<ScanRecord>, DomainError>;
    fn delete_scan(&self, id: &ScanId) -> Result<(), DomainError>;
    fn exists(&self, id: &ScanId) -> Result<bool, DomainError>;
    fn count(&self) -> Result<usize, DomainError>;
}

pub trait AuditLogRepository: Send + Sync {
    fn append_log(&self, entry: &AuditLogEntry) -> Result<(), DomainError>;
}
