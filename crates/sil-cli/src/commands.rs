use sil_policy::rules::RiskInput;

/// Builds a [`RiskInput`] from confusable detection flags.
///
/// - If confusable flags are present, `unicode_risk` is set to 0.8 and
///   `confusable_risk` to 0.9.
/// - `semantic_risk` is neutral (0.0) since the CLI has no reference input.
pub fn build_risk_input(confusable_flags: &[String]) -> RiskInput {
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
        semantic_risk: 0.0,
    }
}
