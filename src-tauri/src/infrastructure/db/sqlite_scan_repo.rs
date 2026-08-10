use super::ConnectionProvider;
use crate::domain::errors::{DomainError, Result};
use crate::domain::persistence::{ScanId, ScanRecord, ScanReport, ScanRepository};
use crate::domain::types::{RiskScore, Severity};
use chrono::{TimeZone, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct SqliteScanRepository {
    provider: Arc<dyn ConnectionProvider>,
}

impl SqliteScanRepository {
    pub fn new(provider: Arc<dyn ConnectionProvider>) -> Self {
        Self { provider }
    }
}

impl ScanRepository for SqliteScanRepository {
    fn save_scan(&self, record: &ScanRecord) -> Result<()> {
        let conn = self.provider.get_connection()?;

        let timestamp_i64 = record.scan_timestamp_utc.timestamp();
        let risk_f64 = record.risk_score.value();
        let severity_str = format!("{:?}", record.severity);

        let json_blob = serde_json::to_string(&record.report)
            .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;

        conn.execute(
            "INSERT INTO scans (id, extension_id, extension_name, version, risk_score, severity, timestamp_utc, raw_json) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                record.id.0.to_string(),
                record.extension_id,
                record.extension_name,
                record.version,
                risk_f64,
                severity_str,
                timestamp_i64,
                json_blob
            ],
        ).map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

        Ok(())
    }

    fn get_scan(&self, id: &ScanId) -> Result<Option<ScanRecord>> {
        let conn = self.provider.get_connection()?;

        let mut stmt = conn.prepare("SELECT id, extension_id, extension_name, version, risk_score, severity, timestamp_utc, raw_json FROM scans WHERE id = ?1")
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

        let mut rows = stmt
            .query(rusqlite::params![id.0.to_string()])
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?
        {
            let id_str: String = row
                .get(0)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let extension_id: String = row
                .get(1)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let extension_name: String = row
                .get(2)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let version: String = row
                .get(3)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let risk_score_f64: f64 = row
                .get(4)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let severity_str: String = row
                .get(5)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let timestamp_i64: i64 = row
                .get(6)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
            let raw_json: String = row
                .get(7)
                .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

            let uuid = Uuid::parse_str(&id_str)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;
            let risk_score = RiskScore::new(risk_score_f64)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;

            let severity = match severity_str.as_str() {
                "Low" => Severity::Low,
                "Medium" => Severity::Medium,
                "High" => Severity::High,
                "Critical" => Severity::Critical,
                _ => Severity::Low,
            };

            let scan_timestamp_utc = Utc
                .timestamp_opt(timestamp_i64, 0)
                .single()
                .unwrap_or_else(Utc::now);

            let report: ScanReport = serde_json::from_str(&raw_json)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;

            Ok(Some(ScanRecord {
                id: ScanId(uuid),
                extension_id,
                extension_name,
                version,
                risk_score,
                severity,
                scan_timestamp_utc,
                report,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_recent_scans(&self, limit: usize) -> Result<Vec<ScanRecord>> {
        let conn = self.provider.get_connection()?;

        let mut stmt = conn.prepare("SELECT id, extension_id, extension_name, version, risk_score, severity, timestamp_utc, raw_json FROM scans ORDER BY timestamp_utc DESC LIMIT ?1")
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

        let row_iter = stmt
            .query_map(rusqlite::params![limit as i64], |row| {
                let id_str: String = row.get(0)?;
                let extension_id: String = row.get(1)?;
                let extension_name: String = row.get(2)?;
                let version: String = row.get(3)?;
                let risk_score_f64: f64 = row.get(4)?;
                let severity_str: String = row.get(5)?;
                let timestamp_i64: i64 = row.get(6)?;
                let raw_json: String = row.get(7)?;
                Ok((
                    id_str,
                    extension_id,
                    extension_name,
                    version,
                    risk_score_f64,
                    severity_str,
                    timestamp_i64,
                    raw_json,
                ))
            })
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

        let mut results = Vec::new();
        for row_result in row_iter {
            let (
                id_str,
                extension_id,
                extension_name,
                version,
                risk_score_f64,
                severity_str,
                timestamp_i64,
                raw_json,
            ) = row_result.map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;

            let uuid = Uuid::parse_str(&id_str)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;
            let risk_score = RiskScore::new(risk_score_f64)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;
            let severity = match severity_str.as_str() {
                "Low" => Severity::Low,
                "Medium" => Severity::Medium,
                "High" => Severity::High,
                "Critical" => Severity::Critical,
                _ => Severity::Low,
            };
            let scan_timestamp_utc = Utc
                .timestamp_opt(timestamp_i64, 0)
                .single()
                .unwrap_or_else(Utc::now);

            let report: ScanReport = serde_json::from_str(&raw_json)
                .map_err(|e| DomainError::DatabaseSerialization(e.to_string()))?;

            results.push(ScanRecord {
                id: ScanId(uuid),
                extension_id,
                extension_name,
                version,
                risk_score,
                severity,
                scan_timestamp_utc,
                report,
            });
        }

        Ok(results)
    }

    fn delete_scan(&self, id: &ScanId) -> Result<()> {
        let conn = self.provider.get_connection()?;
        conn.execute(
            "DELETE FROM scans WHERE id = ?1",
            rusqlite::params![id.0.to_string()],
        )
        .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
        Ok(())
    }

    fn exists(&self, id: &ScanId) -> Result<bool> {
        let conn = self.provider.get_connection()?;
        let mut stmt = conn
            .prepare("SELECT 1 FROM scans WHERE id = ?1 LIMIT 1")
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
        let mut rows = stmt
            .query(rusqlite::params![id.0.to_string()])
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
        Ok(rows
            .next()
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?
            .is_some())
    }

    fn count(&self) -> Result<usize> {
        let conn = self.provider.get_connection()?;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM scans", [], |row| row.get(0))
            .map_err(|e| DomainError::DatabaseQuery(e.to_string()))?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::risk_calculator::RiskAssessment;
    use crate::domain::risk_calculator::ScoreExplanation;
    use rusqlite::Connection;
    use std::sync::Mutex;

    struct MockConnectionProvider {
        conn: Mutex<Connection>,
    }

    impl MockConnectionProvider {
        fn new() -> Self {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute(
                "CREATE TABLE scans (
                    id TEXT PRIMARY KEY,
                    extension_id TEXT NOT NULL,
                    extension_name TEXT NOT NULL,
                    version TEXT NOT NULL,
                    risk_score REAL NOT NULL,
                    severity TEXT NOT NULL,
                    timestamp_utc INTEGER NOT NULL,
                    raw_json TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
            Self {
                conn: Mutex::new(conn),
            }
        }
    }

    impl ConnectionProvider for MockConnectionProvider {
        fn get_connection(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
            self.conn
                .lock()
                .map_err(|_| DomainError::DatabaseConnection("poisoned".into()))
        }
    }

    fn make_record() -> ScanRecord {
        ScanRecord {
            id: ScanId(Uuid::new_v4()),
            extension_id: "ext123".to_string(),
            extension_name: "Test Ext".to_string(),
            version: "1.0.0".to_string(),
            risk_score: RiskScore::new(45.0).unwrap(),
            severity: Severity::Medium,
            scan_timestamp_utc: Utc::now(),
            report: ScanReport {
                risk_assessment: RiskAssessment {
                    assessment_id: "test-id".to_string(),
                    raw_score: 45.0,
                    normalized_score: RiskScore::new(45.0).unwrap(),
                    severity: Severity::Medium,
                    breakdown: vec![],
                    explanation: ScoreExplanation {
                        top_contributors: vec![],
                        breakdown_count: 0,
                    },
                    diagnostics: vec![],
                },
                findings_count: 0,
            },
        }
    }

    #[test]
    fn test_save_and_get_scan() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteScanRepository::new(provider);

        let record = make_record();
        repo.save_scan(&record).expect("Failed to save scan");

        let fetched = repo
            .get_scan(&record.id)
            .expect("Failed to get scan")
            .expect("Scan not found");
        assert_eq!(fetched.id.0, record.id.0);
        assert_eq!(fetched.extension_id, "ext123");
        assert_eq!(fetched.risk_score.value(), 45.0);
    }

    #[test]
    fn test_exists_and_delete() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteScanRepository::new(provider);

        let record = make_record();
        repo.save_scan(&record).unwrap();

        assert!(repo.exists(&record.id).unwrap());

        repo.delete_scan(&record.id).unwrap();

        assert!(!repo.exists(&record.id).unwrap());
    }

    #[test]
    fn test_count_and_recent() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteScanRepository::new(provider);

        for _ in 0..5 {
            repo.save_scan(&make_record()).unwrap();
        }

        assert_eq!(repo.count().unwrap(), 5);

        let recent = repo.get_recent_scans(3).unwrap();
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_get_missing_scan() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteScanRepository::new(provider);

        let result = repo.get_scan(&ScanId(Uuid::new_v4())).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_database_constraint_error() {
        let provider = Arc::new(MockConnectionProvider::new());
        let repo = SqliteScanRepository::new(provider);

        let record = make_record();
        repo.save_scan(&record).unwrap();

        // Saving same ID should cause constraint violation, bubbled up via DatabaseQuery mapping
        let result = repo.save_scan(&record);
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::DatabaseQuery(err) => assert!(err.contains("UNIQUE constraint failed")),
            _ => panic!("Expected DatabaseQuery error"),
        }
    }
}
