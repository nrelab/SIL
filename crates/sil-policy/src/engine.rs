use crate::decision::Decision;
use crate::error::PolicyError;
use crate::rules::{RiskInput, evaluate_risk};

/// Evaluates risk input against thresholds and produces a [`Decision`].
///
/// # Thresholds
/// - Score > 0.8 → [`Block`](Decision::Block)
/// - Score > 0.5 → [`Warn`](Decision::Warn)
/// - Repairable issue detected → [`Rewrite`](Decision::Rewrite)
/// - Otherwise → [`Allow`](Decision::Allow)
///
/// `original` is the raw input text, used to check for repairable issues
/// (zero-width characters and the f-hook character).
///
/// # Errors
///
/// Returns [`PolicyError::InvalidRiskScore`] if any risk dimension in
/// `input` is NaN, infinite, or outside `0.0..=1.0`.
pub fn evaluate(input: &RiskInput, original: &str) -> Result<Decision, PolicyError> {
    let score = evaluate_risk(input)?;

    if score > 0.8 {
        return Ok(Decision::Block);
    }

    if score > 0.5 {
        return Ok(Decision::Warn);
    }

    if contains_repairable_issue(original) {
        return Ok(Decision::Rewrite(sanitize(original)));
    }

    Ok(Decision::Allow)
}

fn contains_repairable_issue(input: &str) -> bool {
    input.contains('\u{200B}') || input.contains('\u{0192}')
}

fn sanitize(input: &str) -> String {
    input.replace('\u{0192}', "f").replace('\u{200B}', "")
}
