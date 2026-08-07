use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::risk_calculator::*;
use crate::domain::rule_matcher::StructuralFinding;
use crate::domain::risk::RiskProfile;
use crate::domain::types::RiskScore;
use crate::domain::rules::AggregationPolicy;

pub struct RiskCalculatorService {
    rule_configs: HashMap<String, RiskRuleConfig>,
    profile: RiskProfile,
}

impl RiskCalculatorService {
    pub fn new(configs: Vec<RiskRuleConfig>, profile: RiskProfile) -> Self {
        let mut map = HashMap::new();
        for cfg in configs {
            map.insert(cfg.rule_id.clone(), cfg);
        }
        Self {
            rule_configs: map,
            profile,
        }
    }

    pub fn calculate(&self, findings: &[StructuralFinding]) -> RiskAssessment {
        let mut raw_score = 0.0;
        let mut diagnostics = Vec::new();
        
        let mut grouped_findings: HashMap<String, Vec<&StructuralFinding>> = HashMap::new();
        for finding in findings {
            grouped_findings.entry(finding.rule_id.0.to_string()).or_default().push(finding);
        }

        let mut breakdowns = Vec::new();

        for (rule_id, group) in grouped_findings {
            let config_opt = self.rule_configs.get(&rule_id);
            if let Some(config) = config_opt {
                let n = group.len();
                let w = config.base_weight;

                let cumulative_weight = match &config.policy {
                    AggregationPolicy::Once => w,
                    AggregationPolicy::Sum => w * (n as f64),
                    AggregationPolicy::Max => w,
                    AggregationPolicy::Decay => {
                        w * 2.0 * (1.0 - (0.5_f64).powi(n as i32))
                    }
                };

                raw_score += cumulative_weight;

                let associated_finding_ids = group.iter().map(|f| f.finding_id.0.clone()).collect();
                
                breakdowns.push(RiskBreakdown {
                    rule_id: rule_id.clone(),
                    findings_count: n,
                    applied_policy: config.policy.clone(),
                    cumulative_weight,
                    associated_finding_ids,
                });
            } else {
                diagnostics.push(format!("WARN: Missing configuration for RuleId({})", rule_id));
            }
        }

        // Round to 2 decimal places before clamping
        let rounded_raw = (raw_score * 100.0).round() / 100.0;
        let clamped_val = rounded_raw.min(100.0).max(0.0);
        let normalized_score = RiskScore::new(clamped_val).unwrap_or_default();
        
        let severity = self.profile.classify(normalized_score);

        // Sort breakdowns descending by cumulative_weight, tie-breaker: rule_id ASC
        breakdowns.sort_by(|a, b| {
            b.cumulative_weight.partial_cmp(&a.cumulative_weight)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.rule_id.cmp(&b.rule_id))
        });

        let breakdown_count = breakdowns.len();
        let top_contributors = breakdowns.iter().take(3).cloned().collect();

        RiskAssessment {
            assessment_id: Uuid::new_v4().to_string(),
            raw_score: rounded_raw,
            normalized_score,
            severity,
            breakdown: breakdowns,
            explanation: ScoreExplanation {
                top_contributors,
                breakdown_count,
            },
            diagnostics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rule_matcher::{FindingId, DetectorKind, RuleId};
    use crate::domain::Severity;

    fn make_finding(rule_id: u32) -> StructuralFinding {
        StructuralFinding {
            finding_id: FindingId(format!("FND-{}", rule_id)),
            rule_id: RuleId(rule_id),
            target_detector: DetectorKind::Any,
            evidence: vec![],
            evidence_count: 1,
        }
    }

    #[test]
    fn test_decay_aggregation_asymptote() {
        let configs = vec![
            RiskRuleConfig {
                rule_id: "10".to_string(),
                base_weight: 10.0,
                policy: AggregationPolicy::Decay,
            }
        ];
        let service = RiskCalculatorService::new(configs, RiskProfile::Default);
        
        let mut findings = vec![];
        for _ in 0..10 {
            findings.push(make_finding(10));
        }

        let assessment = service.calculate(&findings);
        // Formula: 10 * 2.0 * (1 - 0.5^10) = 20 * (1 - 0.0009765625) = 19.98046875 -> rounded to 19.98
        assert_eq!(assessment.raw_score, 19.98);
        assert_eq!(assessment.breakdown[0].cumulative_weight, 19.98046875);
    }

    #[test]
    fn test_sum_ceiling_clamping() {
        let configs = vec![
            RiskRuleConfig {
                rule_id: "20".to_string(),
                base_weight: 50.0,
                policy: AggregationPolicy::Sum,
            }
        ];
        let service = RiskCalculatorService::new(configs, RiskProfile::Default);
        
        let mut findings = vec![];
        for _ in 0..7 { // 7 * 50 = 350.0
            findings.push(make_finding(20));
        }

        let assessment = service.calculate(&findings);
        assert_eq!(assessment.raw_score, 350.0);
        assert_eq!(assessment.normalized_score.value(), 100.0);
        assert_eq!(assessment.severity, Severity::Critical);
    }

    #[test]
    fn test_max_and_once_aggregation() {
        let configs = vec![
            RiskRuleConfig { rule_id: "30".to_string(), base_weight: 45.0, policy: AggregationPolicy::Max },
            RiskRuleConfig { rule_id: "40".to_string(), base_weight: 12.0, policy: AggregationPolicy::Once },
        ];
        let service = RiskCalculatorService::new(configs, RiskProfile::Default);
        
        let mut findings = vec![];
        for _ in 0..5 { findings.push(make_finding(30)); }
        for _ in 0..3 { findings.push(make_finding(40)); }

        let assessment = service.calculate(&findings);
        assert_eq!(assessment.raw_score, 57.0); // 45.0 + 12.0
        
        let max_breakdown = assessment.breakdown.iter().find(|b| b.rule_id == "30").unwrap();
        assert_eq!(max_breakdown.cumulative_weight, 45.0);
        
        let once_breakdown = assessment.breakdown.iter().find(|b| b.rule_id == "40").unwrap();
        assert_eq!(once_breakdown.cumulative_weight, 12.0);
    }

    #[test]
    fn test_missing_rule_id_diagnostics() {
        let service = RiskCalculatorService::new(vec![], RiskProfile::Default);
        let findings = vec![make_finding(999)];
        
        let assessment = service.calculate(&findings);
        assert_eq!(assessment.raw_score, 0.0);
        assert_eq!(assessment.breakdown.len(), 0);
        assert_eq!(assessment.diagnostics.len(), 1);
        assert_eq!(assessment.diagnostics[0], "WARN: Missing configuration for RuleId(999)");
    }

    #[test]
    fn test_tie_breaking_order() {
        let configs = vec![
            RiskRuleConfig { rule_id: "300".to_string(), base_weight: 25.0, policy: AggregationPolicy::Once },
            RiskRuleConfig { rule_id: "100".to_string(), base_weight: 25.0, policy: AggregationPolicy::Once },
            RiskRuleConfig { rule_id: "200".to_string(), base_weight: 25.0, policy: AggregationPolicy::Once },
        ];
        let service = RiskCalculatorService::new(configs, RiskProfile::Default);
        
        let findings = vec![make_finding(100), make_finding(200), make_finding(300)];
        let assessment = service.calculate(&findings);
        
        // Expected order: 100, 200, 300 due to tie breaking on RuleId ASC
        assert_eq!(assessment.explanation.top_contributors[0].rule_id, "100");
        assert_eq!(assessment.explanation.top_contributors[1].rule_id, "200");
        assert_eq!(assessment.explanation.top_contributors[2].rule_id, "300");
    }
    
    #[test]
    fn test_rounding() {
        let configs = vec![
            RiskRuleConfig { rule_id: "1".to_string(), base_weight: 33.333333, policy: AggregationPolicy::Once },
        ];
        let service = RiskCalculatorService::new(configs, RiskProfile::Default);
        let findings = vec![make_finding(1)];
        let assessment = service.calculate(&findings);
        
        assert_eq!(assessment.raw_score, 33.33); // Rounded down
    }
}
