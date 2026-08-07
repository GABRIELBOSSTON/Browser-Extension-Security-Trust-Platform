use std::time::Instant;
use crate::domain::capabilities::ExtensionCapabilityModel;
use crate::domain::rules::{RuleSet, Finding, Evidence};
use crate::application::rules::models::{RuleEvaluationResult, ExecutionStats};
use uuid::Uuid;

pub struct RuleEngine {
    rule_set: RuleSet,
}

impl RuleEngine {
    pub fn new(rule_set: RuleSet) -> Self {
        Self { rule_set }
    }

    pub fn evaluate(&self, model: &ExtensionCapabilityModel) -> RuleEvaluationResult {
        let start = Instant::now();
        let mut findings = Vec::new();
        let mut rules_evaluated = 0;

        for rule in &self.rule_set.rules {
            rules_evaluated += 1;
            
            // Map matcher_id to native Rust matchers
            if let Some(evidence) = Self::execute_matcher(&rule.matcher_id, model) {
                findings.push(Finding {
                    finding_id: Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    severity: rule.severity,
                    description: rule.description.clone(),
                    evidence,
                });
            }
        }

        let execution_time_ms = start.elapsed().as_millis() as u64;

        RuleEvaluationResult {
            findings,
            execution_stats: ExecutionStats {
                execution_time_ms,
                rules_evaluated,
            },
            rule_version: self.rule_set.version.clone(),
        }
    }

    fn execute_matcher(matcher_id: &str, model: &ExtensionCapabilityModel) -> Option<Evidence> {
        match matcher_id {
            "matcher_broad_host" => {
                // Check if the extension requests broad host permissions like <all_urls> or *://*/*
                for pattern in &model.hosts.patterns {
                    if pattern.raw == "<all_urls>" || pattern.raw == "*://*/*" || pattern.raw == "*://*/" {
                        return Some(Evidence::MatchPattern(pattern.clone()));
                    }
                }
                None
            }
            "matcher_debugger_api" => {
                // Check if the extension requests the debugger API
                for perm in &model.permissions.items {
                    if perm.name == "debugger" {
                        return Some(Evidence::Capability(perm.id.clone()));
                    }
                }
                None
            }
            _ => {
                // Unknown matcher, skip
                None
            }
        }
    }
}
