use crate::error::PolicyError;

/// Risk scoring input with three weighted dimensions.
///
/// Each field should be in the range `[0.0, 1.0]`:
/// - `unicode_risk`: weight 0.4
/// - `confusable_risk`: weight 0.4
/// - `semantic_risk`: weight 0.2
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RiskInput {
    pub unicode_risk: f32,
    pub confusable_risk: f32,
    pub semantic_risk: f32,
}

/// Evaluates the overall risk score from a [`RiskInput`].
///
/// Returns a weighted score in `[0.0, 1.0]` calculated as:
/// `(unicode_risk * 0.4) + (confusable_risk * 0.4) + (semantic_risk * 0.2)`
///
/// # Errors
///
/// Returns [`PolicyError::InvalidRiskScore`] if any risk dimension is
/// NaN, infinite, or outside `0.0..=1.0`.
pub fn evaluate_risk(input: &RiskInput) -> Result<f32, PolicyError> {
    for (field, value) in [
        ("unicode_risk", input.unicode_risk),
        ("confusable_risk", input.confusable_risk),
        ("semantic_risk", input.semantic_risk),
    ] {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(PolicyError::InvalidRiskScore { field, value });
        }
    }
    Ok((input.unicode_risk * 0.4) + (input.confusable_risk * 0.4) + (input.semantic_risk * 0.2))
}
