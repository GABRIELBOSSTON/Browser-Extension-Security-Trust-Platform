use crate::domain::errors::Result;
use crate::domain::persistence::{
    AuditAction, AuditLogEntry, AuditLogRepository, AuditMetadata, ScanRecord, ScanRepository,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct PersistenceService {
    scan_repo: Arc<dyn ScanRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
}

impl PersistenceService {
    pub fn new(
        scan_repo: Arc<dyn ScanRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
    ) -> Self {
        Self {
            scan_repo,
            audit_repo,
        }
    }

    pub fn archive_scan(&self, record: ScanRecord) -> Result<()> {
        let mut metadata = HashMap::new();
        metadata.insert("extension_id".to_string(), record.extension_id.clone());
        metadata.insert("scan_id".to_string(), record.id.0.to_string());

        let audit_entry = AuditLogEntry {
            log_id: Uuid::new_v4(),
            action: AuditAction::ScanCompleted,
            metadata: AuditMetadata { metadata },
            timestamp_utc: Utc::now(),
        };

        self.scan_repo.save_scan(&record)?;
        self.audit_repo.append_log(&audit_entry)?;

        Ok(())
    }
}
