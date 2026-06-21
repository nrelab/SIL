/// Risk scoring input with three weighted dimensions.
///
/// Each field should be in the range `[0.0, 1.0]`:
/// - `unicode_risk`: weight 0.4
/// - `confusable_risk`: weight 0.4
/// - `semantic_risk`: weight 0.2
pub struct RiskInput {
    pub unicode_risk: f32,
    pub confusable_risk: f32,
    pub semantic_risk: f32,
}

/// Evaluates the overall risk score from a [`RiskInput`].
///
/// Returns a weighted score in `[0.0, 1.0]` calculated as:
/// `(unicode_risk * 0.4) + (confusable_risk * 0.4) + (semantic_risk * 0.2)`
#[must_use]
pub fn evaluate_risk(input: &RiskInput) -> f32 {
    (input.unicode_risk * 0.4) + (input.confusable_risk * 0.4) + (input.semantic_risk * 0.2)
}
