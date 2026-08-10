use super::models::VirusTotalReport;
use rusqlite::{params, Connection, Result as SqlResult};
use std::path::Path;
use std::sync::Mutex;

pub struct VtCache {
    conn: Mutex<Connection>,
}

impl VtCache {
    /// Initialize the SQLite cache database
    pub fn new<P: AsRef<Path>>(db_path: P) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS vt_cache (
                sha256 TEXT PRIMARY KEY,
                report_json TEXT NOT NULL,
                cached_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Retrieve a cached report for a given hash
    pub fn get_cached(&self, sha256: &str) -> Option<VirusTotalReport> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT report_json, cached_at FROM vt_cache WHERE sha256 = ?1")
            .ok()?;

        let mut rows = stmt.query(params![sha256]).ok()?;
        if let Some(row) = rows.next().ok().flatten() {
            let json: String = row.get(0).ok()?;
            let _cached_at: i64 = row.get(1).ok()?;

            // Check expiry? For now, we trust the cache indefinitely or can add a TTL later.
            return serde_json::from_str(&json).ok();
        }
        None
    }

    /// Store a report in the cache
    pub fn set_cached(&self, sha256: &str, report: &VirusTotalReport) -> SqlResult<()> {
        let json = serde_json::to_string(report).unwrap_or_default();
        let now = chrono::Utc::now().timestamp();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO vt_cache (sha256, report_json, cached_at) VALUES (?1, ?2, ?3)",
            params![sha256, json, now],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_vt_cache_lifecycle() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("vt.db");
        let cache = VtCache::new(db_path).unwrap();

        let mut report = VirusTotalReport::new_empty("hash123");
        report.malicious = 10;

        // Ensure not found initially
        assert!(cache.get_cached("hash123").is_none());

        // Insert and retrieve
        cache.set_cached("hash123", &report).unwrap();
        let cached = cache.get_cached("hash123").unwrap();
        assert_eq!(cached.malicious, 10);
        assert_eq!(cached.sha256, "hash123");
    }
}
