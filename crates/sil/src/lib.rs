#![warn(clippy::pedantic)]
#![doc = "Unified interface for the Semantic Integrity Layer."]
#![doc = ""]
#![doc = "Re-exports the public API of all SIL sub-crates for convenience."]
#![doc = "Add `sil` to your `Cargo.toml` instead of depending on individual"]
#![doc = "sub-crates."]

/// Detect confusable Unicode patterns and compute risk scores.
pub use sil_confusable::{confusable_score, detect_confusables};
/// Normalize Unicode input and scan for suspicious patterns.
pub use sil_normalizer::{normalize_input, scan_input};
/// Evaluate risk and make policy decisions.
pub use sil_policy::{evaluate, Decision};
/// Risk input types and scoring (accessible via `sil_policy::rules`).
pub use sil_policy::rules::{evaluate_risk, RiskInput};
/// Compute semantic similarity and cluster intents.
pub use sil_semantic::{cluster_intents, semantic_similarity};
