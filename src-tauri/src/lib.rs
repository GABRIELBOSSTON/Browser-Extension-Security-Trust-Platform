pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use application::manifest::service::ManifestService;
use application::pipeline::AnalysisPipeline;
use application::rules::engine::RuleEngine;
use application::virustotal::VirusTotalEngine;
use domain::rules::RuleSet;
use infrastructure::DatabaseManager;
use presentation::commands::{
    app_version, explain_extension, get_database_status, get_installed_extensions, ping,
    scan_extension, scan_extensions,
};
use presentation::state::AppState;
use std::sync::Arc;
use tracing::info;

pub fn run() {
    tracing_subscriber::fmt::init();
    info!("Starting Antigraviiti Extension Protect (AEP) Desktop Agent Core...");

    let db_manager = DatabaseManager::new(None)
        .expect("Critical Failure: Unable to initialize local SQLite database");
    let db_state = Arc::new(db_manager);

    let vt_db_path = db_state.get_db_path().parent().unwrap().join("vt.db");
    let vt_engine = Arc::new(VirusTotalEngine::new(vt_db_path).expect("Failed to init VT Engine"));

    // Initialize Analysis Services
    let manifest_svc = Arc::new(ManifestService);
    let rule_set = Arc::new(RuleSet::default());
    let rule_engine = Arc::new(RuleEngine::new(RuleSet::default())); // Using default for now
    let analysis_pipeline = Arc::new(AnalysisPipeline::new(
        manifest_svc,
        rule_engine,
        rule_set,
        vt_engine,
    ));

    let app_state = AppState::new(analysis_pipeline);

    tauri::Builder::default()
        .manage(db_state)
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            ping,
            app_version,
            get_database_status,
            get_installed_extensions,
            scan_extension,
            scan_extensions,
            explain_extension
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
