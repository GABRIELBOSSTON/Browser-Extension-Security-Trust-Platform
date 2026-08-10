use crate::domain::evidence::{CorrelatedRiskResult, EvidenceItem};

pub struct RiskCorrelator;

impl RiskCorrelator {
    /// Correlates independent evidence items into a final risk score using an asymptotic decay model.
    pub fn correlate(mut evidence: Vec<EvidenceItem>) -> CorrelatedRiskResult {
        // Separate positive and negative evidence
        let mut negative_evidence: Vec<&EvidenceItem> = Vec::new();
        let mut positive_score = 0;

        for item in &evidence {
            if item.base_score < 0 {
                positive_score += item.base_score;
            } else {
                negative_evidence.push(item);
            }
        }

        // Sort negative evidence descending by base_score
        negative_evidence.sort_by_key(|b| std::cmp::Reverse(b.base_score));

        // Deduplicate negative evidence by detail string to prevent artificial bloat
        // We only want the highest scoring evidence for a specific detail string
        let mut seen_details = std::collections::HashSet::new();
        let mut unique_negative = Vec::new();
        for item in negative_evidence {
            if seen_details.insert(item.detail.clone()) {
                unique_negative.push(item);
            }
        }

        // Apply decay function
        let mut raw_negative_score: f64 = 0.0;
        let mut multiplier: f64 = 1.0;

        for item in unique_negative {
            raw_negative_score += (item.base_score as f64) * multiplier;
            multiplier *= 0.5; // Each subsequent finding has half the weight
        }

        // Final score calculation
        // Total = Raw Negative + Positive (which is negative in value)
        let total_score = raw_negative_score + (positive_score as f64);

        // Clamp between 0 and 100
        let final_score_f = total_score.clamp(0.0, 100.0);
        let final_score = final_score_f.round() as u32;

        let final_level = match final_score {
            0..=20 => "Safe",
            21..=40 => "Low",
            41..=60 => "Medium",
            61..=80 => "High",
            _ => "Critical",
        }
        .to_string();

        // Sort the final returned evidence vector (Critical first, then High, etc.)
        evidence.sort_by_key(|b| std::cmp::Reverse(b.base_score));

        // We can optionally deduplicate the final evidence list so UI doesn't show duplicates
        let mut final_evidence = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for item in evidence {
            if seen.insert(item.detail.clone()) {
                final_evidence.push(item);
            }
        }

        CorrelatedRiskResult {
            final_score,
            final_level,
            evidence: final_evidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlator_decay_logic() {
        let evidence = vec![
            EvidenceItem {
                category: "AST".to_string(),
                detail: "eval()".to_string(),
                severity: "High".to_string(),
                base_score: 40,
            },
            EvidenceItem {
                category: "AST".to_string(),
                detail: "fetch()".to_string(),
                severity: "Medium".to_string(),
                base_score: 20,
            },
            EvidenceItem {
                category: "AST".to_string(),
                detail: "setTimeout()".to_string(),
                severity: "Low".to_string(),
                base_score: 5,
            },
        ];

        // 40 + (20 * 0.5) + (5 * 0.25) = 40 + 10 + 1.25 = 51.25 -> 51
        let result = RiskCorrelator::correlate(evidence);
        assert_eq!(result.final_score, 51);
        assert_eq!(result.final_level, "Medium");
    }

    #[test]
    fn test_correlator_with_trust() {
        let evidence = vec![
            EvidenceItem {
                category: "AST".to_string(),
                detail: "eval()".to_string(),
                severity: "Critical".to_string(),
                base_score: 80,
            },
            EvidenceItem {
                category: "Trust".to_string(),
                detail: "Trusted Publisher".to_string(),
                severity: "Good".to_string(),
                base_score: -30,
            },
        ];

        // 80 - 30 = 50
        let result = RiskCorrelator::correlate(evidence);
        assert_eq!(result.final_score, 50);
        assert_eq!(result.final_level, "Medium");
    }

    #[test]
    fn test_correlator_trust_never_hides_critical() {
        let evidence = vec![
            EvidenceItem {
                category: "VT".to_string(),
                detail: "Malicious VT".to_string(),
                severity: "Critical".to_string(),
                base_score: 80,
            },
            EvidenceItem {
                category: "IOC".to_string(),
                detail: "Discord webhook".to_string(),
                severity: "Critical".to_string(),
                base_score: 80,
            },
            EvidenceItem {
                category: "Trust".to_string(),
                detail: "Trusted Publisher".to_string(),
                severity: "Good".to_string(),
                base_score: -30,
            },
        ];

        // 80*1.0 + 80*0.5 = 120
        // 120 - 30 = 90
        let result = RiskCorrelator::correlate(evidence);
        assert_eq!(result.final_score, 90);
        assert_eq!(result.final_level, "Critical");
    }
}
