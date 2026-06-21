#![warn(clippy::pedantic)]

pub mod decision;
pub mod engine;
pub mod rules;

pub use decision::Decision;
pub use engine::evaluate;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RiskInput;

    #[test]
    fn test_allow_low_risk() {
        let input = RiskInput {
            unicode_risk: 0.1,
            confusable_risk: 0.1,
            semantic_risk: 0.1,
        };
        assert_eq!(evaluate(&input, "hello"), Decision::Allow);
    }

    #[test]
    fn test_warn_medium_risk() {
        let input = RiskInput {
            unicode_risk: 0.7,
            confusable_risk: 0.7,
            semantic_risk: 0.0,
        };
        let decision = evaluate(&input, "userdev");
        assert!(matches!(decision, Decision::Warn));
    }

    #[test]
    fn test_block_high_risk() {
        let input = RiskInput {
            unicode_risk: 0.9,
            confusable_risk: 0.9,
            semantic_risk: 0.5,
        };
        assert_eq!(evaluate(&input, "раypal"), Decision::Block);
    }

    #[test]
    fn test_rewrite_repairable() {
        let input = RiskInput {
            unicode_risk: 0.1,
            confusable_risk: 0.1,
            semantic_risk: 0.1,
        };
        let decision = evaluate(&input, "\u{0192}dev");
        assert_eq!(decision, Decision::Rewrite("fdev".to_string()));
    }

    #[test]
    fn test_score_above_08_is_block() {
        let input = RiskInput {
            unicode_risk: 1.0,
            confusable_risk: 1.0,
            semantic_risk: 1.0,
        };
        let score = rules::evaluate_risk(&input);
        assert!(score > 0.8);
    }

    #[test]
    fn test_score_weighted_correctly() {
        let input = RiskInput {
            unicode_risk: 1.0,
            confusable_risk: 0.0,
            semantic_risk: 0.0,
        };
        let score = rules::evaluate_risk(&input);
        assert_eq!(score, 0.4);
    }
}
