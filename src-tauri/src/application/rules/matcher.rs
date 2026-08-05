use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::domain::rule_matcher::*;

pub struct RuleMatcherService {
    repository: Arc<dyn RuleRepository>,
}

impl RuleMatcherService {
    pub fn new(repository: Arc<dyn RuleRepository>) -> Self {
        Self { repository }
    }

    pub fn evaluate(&self, inventories: &HashMap<DetectorKind, Arc<dyn DetectorInventory>>) -> RuleMatcherResult {
        let start_time = Instant::now();
        let rules = self.repository.fetch_active_rules();
        
        let mut findings = Vec::new();
        let mut conditions_checked = 0;
        let mut rules_evaluated = 0;

        for rule in rules {
            if !rule.enabled {
                continue;
            }
            rules_evaluated += 1;
            
            let mut rule_matched = true;
            let mut current_evidence = Vec::new();

            for condition in &rule.conditions {
                conditions_checked += 1;
                
                // If inventory is missing for the target detector, rule match automatically fails
                let inventory_opt = inventories.get(&condition.target_detector);
                if let Some(inventory) = inventory_opt {
                    let is_condition_met = match &condition.condition_type {
                        ConditionType::Exists => inventory.exists(condition.expected_id),
                        ConditionType::NotExists => !inventory.exists(condition.expected_id),
                        ConditionType::Equals => inventory.get_match_count(condition.expected_id) > 0, // Simplified
                        ConditionType::NotEquals => inventory.get_match_count(condition.expected_id) == 0,
                        ConditionType::CountGreaterThan(threshold) => inventory.get_match_count(condition.expected_id) > *threshold,
                        ConditionType::CountLessThan(threshold) => inventory.get_match_count(condition.expected_id) < *threshold,
                        ConditionType::Contains(_) | ConditionType::StartsWith(_) => {
                            // Requires expanding DetectorInventory trait to pass string queries
                            // For this structural implementation, fallback to false if unsupported
                            false 
                        }
                    };

                    if is_condition_met {
                        current_evidence.push(EvidenceReference {
                            reference_id: condition.expected_id,
                            description: format!("Condition {:?} met for ID {}", condition.condition_type, condition.expected_id),
                        });
                    } else {
                        rule_matched = false;
                        break;
                    }
                } else {
                    rule_matched = false;
                    break;
                }
            }

            if rule_matched && !current_evidence.is_empty() {
                findings.push(StructuralFinding {
                    finding_id: FindingId(format!("FND-{}-{}", rule.rule_id.0, start_time.elapsed().as_micros())),
                    rule_id: rule.rule_id,
                    target_detector: rule.conditions.first().map(|c| c.target_detector.clone()).unwrap_or(DetectorKind::Any),
                    evidence_count: current_evidence.len(),
                    evidence: current_evidence,
                });
            }
        }

        let elapsed_ms = start_time.elapsed().as_millis() as u64;

        RuleMatcherResult {
            matched_rules: findings.len(), // Number of rules that produced a finding
            statistics: ExecutionStatistics {
                rules_evaluated,
                conditions_checked,
                finding_count: findings.len(),
            },
            findings,
            elapsed_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInventory {
        kind: DetectorKind,
        matches: HashMap<u32, usize>,
    }

    impl DetectorInventory for MockInventory {
        fn detector_kind(&self) -> DetectorKind {
            self.kind.clone()
        }

        fn get_match_count(&self, expected_id: u32) -> usize {
            *self.matches.get(&expected_id).unwrap_or(&0)
        }

        fn exists(&self, expected_id: u32) -> bool {
            self.get_match_count(expected_id) > 0
        }
    }

    struct MockRuleRepository {
        rules: Vec<RuleDefinition>,
    }

    impl RuleRepository for MockRuleRepository {
        fn fetch_active_rules(&self) -> Vec<RuleDefinition> {
            self.rules.clone()
        }
    }

    #[test]
    fn test_rule_evaluation() {
        let mut chrome_matches = HashMap::new();
        chrome_matches.insert(1001, 1);
        chrome_matches.insert(1002, 5);
        
        let chrome_inv = Arc::new(MockInventory {
            kind: DetectorKind::ChromeApi,
            matches: chrome_matches,
        }) as Arc<dyn DetectorInventory>;

        let mut inventories = HashMap::new();
        inventories.insert(DetectorKind::ChromeApi, chrome_inv);

        let rules = vec![
            RuleDefinition {
                rule_id: RuleId(1),
                name: "Test Rule 1".to_string(),
                description: "Test".to_string(),
                enabled: true,
                version: "1.0".to_string(),
                tags: vec![],
                conditions: vec![
                    RuleCondition {
                        target_detector: DetectorKind::ChromeApi,
                        expected_id: 1001,
                        condition_type: ConditionType::Exists,
                    },
                ],
            },
            RuleDefinition {
                rule_id: RuleId(2),
                name: "Test Rule 2".to_string(),
                description: "Test".to_string(),
                enabled: true,
                version: "1.0".to_string(),
                tags: vec![],
                conditions: vec![
                    RuleCondition {
                        target_detector: DetectorKind::ChromeApi,
                        expected_id: 1002,
                        condition_type: ConditionType::CountGreaterThan(3),
                    },
                ],
            }
        ];

        let repo = Arc::new(MockRuleRepository { rules });
        let service = RuleMatcherService::new(repo);

        let result = service.evaluate(&inventories);
        
        assert_eq!(result.matched_rules, 2);
        assert_eq!(result.statistics.rules_evaluated, 2);
        assert_eq!(result.statistics.conditions_checked, 2);
    }
    
    #[test]
    fn test_cross_detector_matching_missing_inventory() {
        let rules = vec![
            RuleDefinition {
                rule_id: RuleId(1),
                name: "Missing Detector Rule".to_string(),
                description: "Test".to_string(),
                enabled: true,
                version: "1.0".to_string(),
                tags: vec![],
                conditions: vec![
                    RuleCondition {
                        target_detector: DetectorKind::DangerousApi, // Not in the map
                        expected_id: 999,
                        condition_type: ConditionType::Exists,
                    },
                ],
            }
        ];
        
        let repo = Arc::new(MockRuleRepository { rules });
        let service = RuleMatcherService::new(repo);
        
        // Empty inventories
        let result = service.evaluate(&HashMap::new());
        assert_eq!(result.matched_rules, 0);
    }
}
