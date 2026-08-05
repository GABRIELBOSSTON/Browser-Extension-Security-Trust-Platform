use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::rules::{RuleSet, AggregationPolicy, Finding};
use crate::domain::types::RiskScore;
use crate::domain::risk::{RiskProfile, RiskAssessment, RiskBreakdown, ScoreExplanation};
use crate::application::rules::RuleEvaluationResult;

pub struct RiskEngine;

impl RiskEngine {
    pub fn assess(
        result: &RuleEvaluationResult,
        rule_set: &RuleSet,
        profile: &RiskProfile,
    ) -> RiskAssessment {
        // Group findings by RuleId
        let mut grouped_findings: HashMap<String, Vec<&Finding>> = HashMap::new();
        for finding in &result.findings {
            grouped_findings
                .entry(finding.rule_id.clone())
                .or_default()
                .push(finding);
        }

        let mut breakdowns = Vec::new();
        let mut raw_score = 0.0;

        for (rule_id, findings) in grouped_findings {
            // Find the original rule to get its weight and aggregation policy
            let rule = rule_set.rules.iter().find(|r| r.rule_id == rule_id);
            if let Some(rule) = rule {
                let base_weight = rule.weight;
                let policy = &rule.aggregation_policy;
                
                let count = findings.len();
                let applied_weight = match policy {
                    AggregationPolicy::Once => base_weight,
                    AggregationPolicy::Sum => base_weight * (count as f64),
                    AggregationPolicy::Max => base_weight, // Assuming findings of same rule have same max base weight here
                    AggregationPolicy::Decay => {
                        let mut total = base_weight;
                        let mut current_weight = base_weight;
                        for _ in 1..count {
                            current_weight *= 0.5; // decay by half each time
                            total += current_weight;
                        }
                        total
                    }
                };

                raw_score += applied_weight;

                let evidence = findings.iter().map(|f| f.evidence.clone()).collect();
                breakdowns.push(RiskBreakdown {
                    rule_id,
                    applied_weight,
                    evidence,
                });
            }
        }

        let normalized_val = raw_score.min(100.0).max(0.0);
        let normalized_score = RiskScore::new(normalized_val).unwrap_or_default();
        let severity = profile.classify(normalized_score);

        // Sort breakdowns descending by applied_weight for the explanation
        breakdowns.sort_by(|a, b| b.applied_weight.partial_cmp(&a.applied_weight).unwrap_or(std::cmp::Ordering::Equal));
        
        let top_contributors = breakdowns.iter().take(3).cloned().collect();
        let explanation = ScoreExplanation { top_contributors };

        RiskAssessment {
            assessment_id: Uuid::new_v4().to_string(),
            raw_score,
            normalized_score,
            severity,
            breakdown: breakdowns,
            explanation,
        }
    }
}
