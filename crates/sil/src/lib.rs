#![warn(clippy::pedantic)]

pub use sil_confusable::{confusable_score, detect_confusables};
pub use sil_normalizer::{normalize_input, scan_input};
pub use sil_policy::{evaluate, Decision};
pub use sil_policy::rules::{evaluate_risk, RiskInput};
pub use sil_semantic::{cluster_intents, semantic_similarity};
