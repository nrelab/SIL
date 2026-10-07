#![warn(clippy::pedantic)]
#![doc = "Unified interface for the Semantic Integrity Layer."]
#![doc = ""]
#![doc = "Re-exports the public API of all SIL sub-crates for convenience."]
#![doc = "Add `sil` to your `Cargo.toml` instead of depending on individual"]
#![doc = "sub-crates."]

/// Detect confusable Unicode patterns and compute risk scores.
pub use sil_confusable::{ConfusableError, confusable_score, detect_confusables, to_ascii_equivalent};
/// Normalize Unicode input and scan for suspicious patterns.
pub use sil_normalizer::{NormalizerError, normalize_input, scan_input};
/// Evaluate risk and make policy decisions.
pub use sil_policy::{Decision, PolicyError, RiskInput, evaluate, evaluate_risk};
/// Compute semantic similarity and cluster intents.
pub use sil_semantic::{SemanticError, cluster_intents, semantic_similarity};
