use crate::decision::Decision;
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
#[must_use]
pub fn evaluate(input: &RiskInput, original: &str) -> Decision {
    let score = evaluate_risk(input);

    if score > 0.8 {
        return Decision::Block;
    }

    if score > 0.5 {
        return Decision::Warn;
    }

    if contains_repairable_issue(original) {
        return Decision::Rewrite(sanitize(original));
    }

    Decision::Allow
}

fn contains_repairable_issue(input: &str) -> bool {
    input.contains('\u{200B}') || input.contains('\u{0192}')
}

fn sanitize(input: &str) -> String {
    input.replace('\u{0192}', "f")
}
