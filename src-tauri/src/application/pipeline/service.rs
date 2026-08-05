use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::errors::{Result, DomainError};
use crate::domain::entities::DiscoveredExtension;
use crate::domain::risk::RiskProfile;
use crate::domain::rules::RuleSet;
use crate::application::manifest::service::ManifestService;
use crate::application::analysis::builder::CapabilityBuilder;
use crate::application::rules::engine::RuleEngine;
use crate::application::risk::engine::RiskEngine;

use super::models::{AnalysisContext, PipelineResult, BatchPipelineResult, PipelineMetadata, StageResult};

pub struct AnalysisPipeline {
    manifest_service: Arc<ManifestService>,
    rule_engine: Arc<RuleEngine>,
    rule_set: Arc<RuleSet>,
}

impl AnalysisPipeline {
    pub fn new(
        manifest_service: Arc<ManifestService>,
        rule_engine: Arc<RuleEngine>,
        rule_set: Arc<RuleSet>,
    ) -> Self {
        Self {
            manifest_service,
            rule_engine,
            rule_set,
        }
    }

    pub async fn analyze_single(
        &self,
        target: &DiscoveredExtension,
        risk_profile: &RiskProfile,
        cancel_token: CancellationToken,
    ) -> Result<PipelineResult> {
        let pipeline_id = Uuid::new_v4().to_string();
        let started_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        let pipeline_start = Instant::now();

        // Clone Arcs and structs to move into spawn_blocking
        let manifest_svc = self.manifest_service.clone();
        let rule_engine = self.rule_engine.clone();
        let rule_set = self.rule_set.clone();
        let target_clone = target.clone();
        let profile_clone = risk_profile.clone();

        let result = tokio::task::spawn_blocking(move || -> Result<PipelineResult> {
            let mut context = AnalysisContext::new(target_clone.clone());
            let mut stage_results = Vec::new();

            // Stage 1: Manifest Parser
            let stage_start = Instant::now();
            if cancel_token.is_cancelled() { return Err(DomainError::IoError("Cancelled".to_string())); }
            let manifest = crate::application::manifest::service::ManifestService::load_manifest(std::path::Path::new(&target_clone.install_path))?;
            context.manifest = Some(manifest);
            stage_results.push(StageResult {
                stage_name: "ManifestParser".to_string(),
                status: "Success".to_string(),
                elapsed_ms: stage_start.elapsed().as_millis() as u64,
                warning_count: 0,
                error: None,
            });

            // Stage 2: Capability Builder
            let stage_start = Instant::now();
            if cancel_token.is_cancelled() { return Err(DomainError::IoError("Cancelled".to_string())); }
            let manifest_ref = context.manifest.as_ref()
                .ok_or_else(|| DomainError::IoError("Manifest is missing".to_string()))?;
            let cap_result = CapabilityBuilder::build(manifest_ref)?;
            context.capability_model = Some(cap_result.model);
            stage_results.push(StageResult {
                stage_name: "CapabilityBuilder".to_string(),
                status: "Success".to_string(),
                elapsed_ms: stage_start.elapsed().as_millis() as u64,
                warning_count: cap_result.warnings.len(),
                error: None,
            });
            context.warnings.extend(cap_result.warnings);

            // Stage 3: Rule Engine
            let stage_start = Instant::now();
            if cancel_token.is_cancelled() { return Err(DomainError::IoError("Cancelled".to_string())); }
            let cap_ref = context.capability_model.as_ref()
                .ok_or_else(|| DomainError::IoError("Capability model is missing".to_string()))?;
            let rule_result = rule_engine.evaluate(cap_ref);
            context.rule_evaluation = Some(rule_result.clone());
            stage_results.push(StageResult {
                stage_name: "RuleEngine".to_string(),
                status: "Success".to_string(),
                elapsed_ms: stage_start.elapsed().as_millis() as u64,
                warning_count: 0,
                error: None,
            });

            // Stage 4: Risk Engine
            let stage_start = Instant::now();
            if cancel_token.is_cancelled() { return Err(DomainError::IoError("Cancelled".to_string())); }
            let risk_assessment = RiskEngine::assess(&rule_result, &rule_set, &profile_clone);
            context.risk_assessment = Some(risk_assessment.clone());
            stage_results.push(StageResult {
                stage_name: "RiskEngine".to_string(),
                status: "Success".to_string(),
                elapsed_ms: stage_start.elapsed().as_millis() as u64,
                warning_count: 0,
                error: None,
            });

            let finished_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
            let elapsed_ms = pipeline_start.elapsed().as_millis() as u64;

            let metadata = PipelineMetadata {
                pipeline_id,
                pipeline_version: env!("CARGO_PKG_VERSION").to_string(),
                started_at,
                finished_at,
                elapsed_ms,
                engine_versions: HashMap::new(),
                os: std::env::consts::OS.to_string(),
                browser: target_clone.browser_family.clone() as u8 as String,
                browser_profile: "default".to_string(),
                rule_set_version: rule_set.version.clone(),
                risk_profile: profile_clone.clone(),
            };

            Ok(PipelineResult {
                assessment: risk_assessment,
                metadata,
                target_info: target_clone.clone(),
                stage_results,
            })
        })
        .await
        .map_err(|e| DomainError::IoError(format!("Task blocking error: {}", e)))??;

        Ok(result)
    }

    pub async fn analyze_batch(
        &self,
        targets: &[DiscoveredExtension],
        risk_profile: &RiskProfile,
        cancel_token: CancellationToken,
    ) -> BatchPipelineResult {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        let total = targets.len();
        let mut success = 0;
        let mut failed = 0;
        let skipped = 0;

        for target in targets {
            if cancel_token.is_cancelled() {
                break;
            }

            match self.analyze_single(target, risk_profile, cancel_token.clone()).await {
                Ok(result) => {
                    results.push(result);
                    success += 1;
                }
                Err(e) => {
                    errors.push(format!("Extension {} failed: {}", target.extension_id, e));
                    failed += 1;
                }
            }
        }

        BatchPipelineResult {
            total,
            success,
            failed,
            skipped,
            results,
            errors,
        }
    }
}
