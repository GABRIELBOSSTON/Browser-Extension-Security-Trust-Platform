use serde::Serialize;
use tauri::State;
use crate::infrastructure::db::ConnectionProvider;
use std::sync::Arc;
use crate::infrastructure::DatabaseManager;
use crate::presentation::state::AppState;
use crate::presentation::models::{ExtensionSummaryResponse, ScanExtensionRequest, ScanExtensionResponse};
use crate::domain::entities::DiscoveredExtension;
use crate::domain::types::{BrowserFamily, BrowserChannel};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Serialize)]
pub struct DatabaseStatusResponse {
    pub connected: bool,
    pub wal_mode_enabled: bool,
    pub version: String,
}

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_database_status(db_state: State<'_, Arc<DatabaseManager>>) -> Result<DatabaseStatusResponse, String> {
    let conn = db_state
        .get_connection()
        .map_err(|e| format!("Database connection error: {}", e))?;

    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode;", [], |row| row.get(0))
        .unwrap_or_else(|_| "unknown".to_string());

    let version: String = conn
        .query_row("SELECT sqlite_version();", [], |row| row.get(0))
        .unwrap_or_else(|_| "SQLite 3".to_string());

    Ok(DatabaseStatusResponse {
        connected: true,
        wal_mode_enabled: journal_mode.to_lowercase() == "wal",
        version: format!("SQLite v{}", version),
    })
}

#[tauri::command]
pub fn get_installed_extensions(_state: State<'_, AppState>) -> Result<Vec<ExtensionSummaryResponse>, String> {
    let result = crate::application::discovery::DiscoveryService::execute_discovery()
        .map_err(|e| e.to_string())?;

    let mut summaries = Vec::new();
    for browser in result.browsers {
        for ext in browser.extensions {
            summaries.push(ExtensionSummaryResponse {
                extension_id: ext.extension_id,
                name: ext.profile_name.clone(), // Map profile to name for now
                version: ext.version,
                browser_family: format!("{:?}", browser.browser_family),
                install_path: ext.install_path,
            });
        }
    }
    
    Ok(summaries)
}

#[tauri::command]
pub async fn scan_extension(
    request: ScanExtensionRequest,
    state: State<'_, AppState>,
) -> Result<ScanExtensionResponse, String> {
    
    // Parse browser family
    let family = match request.browser_family.to_lowercase().as_str() {
        "chrome" => BrowserFamily::Chrome,
        "brave" => BrowserFamily::Brave,
        "edge" => BrowserFamily::Edge,
        "firefox" => BrowserFamily::Firefox,
        _ => BrowserFamily::Unknown,
    };

    let target = DiscoveredExtension {
        extension_id: request.extension_id,
        name: "Unknown Extension".to_string(),
        manifest: None,
        browser_family: family,
        browser_channel: BrowserChannel::Stable, // Default
        profile_name: "Default".to_string(),
        install_path: request.install_path,
        version: "unknown".to_string(),
        disabled: false,
        policy_installed: false,
    };

    let risk_profile = crate::domain::risk::RiskProfile::Default;
    let cancel_token = CancellationToken::new();

    let pipeline = state.analysis_pipeline.clone();
    
    // 1. Offload CPU-heavy pipeline to spawn_blocking in Application Service equivalent
    // NOTE: Application pipeline itself is internally updated to spawn_blocking. 
    // Here we just call it async.
    let result = pipeline.analyze_single(&target, &risk_profile, cancel_token)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ScanExtensionResponse {
        pipeline_id: result.metadata.pipeline_id,
        status: "Completed".to_string(),
        risk_score: result.assessment.normalized_score.value(),
        severity: format!("{:?}", result.assessment.severity),
        elapsed_ms: result.metadata.elapsed_ms,
    })
}

