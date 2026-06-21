use std::collections::HashSet;

/// Computes a semantic similarity score between two strings using Jaccard similarity
/// on underscore-delimited tokens.
///
/// Returns a value in `[0.0, 1.0]` where:
/// - `1.0` indicates identical token sets
/// - `0.0` indicates no overlapping tokens
#[must_use]
pub fn semantic_similarity(a: &str, b: &str) -> f32 {
    jaccard_like_score(a, b)
}

#[allow(clippy::cast_precision_loss)]
fn jaccard_like_score(a: &str, b: &str) -> f32 {
    let set_a: HashSet<_> = a.split('_').collect();
    let set_b: HashSet<_> = b.split('_').collect();

    let intersection = set_a.intersection(&set_b).count() as f32;
    let union = set_a.union(&set_b).count() as f32;

    if union == 0.0 {
        return 0.0;
    }

    intersection / union
}
