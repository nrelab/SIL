use sil_policy::rules::RiskInput;

pub fn build_risk_input(confusable_flags: &[String], semantic_score: f32) -> RiskInput {
    RiskInput {
        unicode_risk: if confusable_flags.is_empty() {
            0.1
        } else {
            0.8
        },
        confusable_risk: if confusable_flags.is_empty() {
            0.1
        } else {
            0.9
        },
        semantic_risk: 1.0 - semantic_score,
    }
}
