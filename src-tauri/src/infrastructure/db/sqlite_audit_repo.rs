use std::sync::Arc;
use uuid::Uuid;
use crate::domain::persistence::{AuditLogRepository, AuditLogEntry};
use crate::domain::errors::{DomainError, Result};
use super::ConnectionProvider;

pub struct SqliteAuditLogRepository {
    provider: Arc<dyn ConnectionProvider>,
}

impl SqliteAuditLogRepository {
    pub fn new(provider: Arc<dyn ConnectionProvider>) -> Self {
        Self { provider }
    }
}

impl AuditLogRepository for SqliteAuditLogRepository {
    fn append_log(&self, entry: &AuditLogEntry) -> Result<()> {
        let conn = self.provider.get_connection()?;
        
        let timestamp_i64 = entry.timestamp_utc.timestamp();
        let action_str = format!("{:?}", entry.action);
        
        let json_blob = serde_json::to_string(&entry.metadata)
            .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;
            
        conn.execute(
            "INSERT INTO audit_logs (log_id, action, metadata, timestamp_utc) 
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                entry.log_id.to_string(), 
                action_str, 
                json_blob, 
                timestamp_i64
            ],
        ).map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::sync::Mutex;
    use chrono::Utc;
    use std::collections::HashMap;
    use crate::domain::persistence::{AuditAction, AuditMetadata};

    struct MockConnectionProvider {
        conn: Mutex<Connection>,
    }

    impl MockConnectionProvider {
        fn new() -> Self {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute(
                "CREATE TABLE audit_logs (
                    log_id TEXT PRIMARY KEY,
                    action TEXT NOT NULL,
                    metadata TEXT NOT NULL,
                    timestamp_utc INTEGER NOT NULL
                )",
                [],
            ).unwrap();
            Self { conn: Mutex::new(conn) }
        }
    }

    impl ConnectionProvider for MockConnectionProvider {
        fn get_connection(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
            self.conn.lock().map_err(|_| DomainError::DatabaseConnection("poisoned".into()))
        }
    }

    #[test]
    fn test_append_log() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteAuditLogRepository::new(provider);
        
        let mut metadata_map = HashMap::new();
        metadata_map.insert("key".to_string(), "val".to_string());

        let entry = AuditLogEntry {
            log_id: Uuid::new_v4(),
            action: AuditAction::ScanCompleted,
            metadata: AuditMetadata { metadata: metadata_map },
            timestamp_utc: Utc::now(),
        };

        repo.append_log(&entry).expect("Failed to append audit log");
    }

    #[test]
    fn test_database_constraint_error() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteAuditLogRepository::new(provider);
        
        let entry = AuditLogEntry {
            log_id: Uuid::new_v4(),
            action: AuditAction::SystemError,
            metadata: AuditMetadata { metadata: HashMap::new() },
            timestamp_utc: Utc::now(),
        };

        repo.append_log(&entry).unwrap();
        
        // Saving same ID should cause constraint violation
        let result = repo.append_log(&entry);
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::DatabaseQuery(err) => assert!(err.contains("UNIQUE constraint failed")),
            _ => panic!("Expected DatabaseQuery error"),
        }
    }
}

