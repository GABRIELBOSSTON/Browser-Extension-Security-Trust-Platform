use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;
use std::sync::MutexGuard;
use crate::domain::errors::{DomainError, Result};

pub mod sqlite_scan_repo;
pub mod sqlite_audit_repo;

/// Abstract Connection Provider
pub trait ConnectionProvider: Send + Sync {
    fn get_connection(&self) -> Result<MutexGuard<'_, Connection>>;
}

/// Database Manager responsible for SQLite initialization, migrations, and WAL mode
pub struct DatabaseManager {
    db_path: PathBuf,
    connection: std::sync::Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(custom_dir: Option<PathBuf>) -> Result<Self> {
        let base_dir = match custom_dir {
            Some(dir) => dir,
            None => {
                let home = dirs::home_dir().ok_or_else(|| {
                    DomainError::IoError("Failed to resolve user home directory".to_string())
                })?;
                home.join(".aep")
            }
        };

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)
                .map_err(|e| DomainError::IoError(format!("Failed to create database directory: {}", e)))?;
        }

        let db_path = base_dir.join("storage.db");
        info!("SQLite Database Path: {:?}", db_path);

        let conn = Connection::open(&db_path)
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to open SQLite database: {}", e)))?;

        let manager = Self { 
            db_path,
            connection: std::sync::Mutex::new(conn),
        };

        manager.init_database()?;
        manager.apply_migrations()?;

        Ok(manager)
    }

    fn init_database(&self) -> Result<()> {
        let conn = self.get_connection()?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to enable WAL mode: {}", e)))?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to set synchronous PRAGMA: {}", e)))?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to enable foreign keys PRAGMA: {}", e)))?;
        info!("SQLite Database initialized successfully with WAL mode enabled.");
        Ok(())
    }

    fn apply_migrations(&self) -> Result<()> {
        let mut conn = self.get_connection()?;
        
        let tx = conn.transaction()
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to start migration transaction: {}", e)))?;

        // Simple migration logic
        tx.execute(
            "CREATE TABLE IF NOT EXISTS scans (
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
        ).map_err(|e| DomainError::DatabaseQuery(format!("Failed to create scans table: {}", e)))?;

        tx.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                log_id TEXT PRIMARY KEY,
                action TEXT NOT NULL,
                metadata TEXT NOT NULL,
                timestamp_utc INTEGER NOT NULL
            )",
            [],
        ).map_err(|e| DomainError::DatabaseQuery(format!("Failed to create audit_logs table: {}", e)))?;

        tx.commit()
            .map_err(|e| DomainError::DatabaseConnection(format!("Failed to commit migrations: {}", e)))?;

        info!("Database schema migrations applied successfully.");
        Ok(())
    }

    pub fn get_db_path(&self) -> &Path {
        &self.db_path
    }
}

impl ConnectionProvider for DatabaseManager {
    fn get_connection(&self) -> Result<MutexGuard<'_, Connection>> {
        self.connection.lock().map_err(|_| DomainError::DatabaseConnection("Mutex poisoned".into()))
    }
}
