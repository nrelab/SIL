#![warn(clippy::pedantic)]
#![doc = "Confusable detection engine for the Semantic Integrity Layer."]
#![doc = ""]
#![doc = "Detects homoglyph attacks, cross-script mixing, and potential"]
#![doc = "impersonation of well-known targets using a configurable"]
#![doc = "Unicode-to-ASCII mapping table."]

/// Module for detecting confusable Unicode patterns in text.
pub mod detector;
/// Module for computing a confusable risk score.
pub mod score;
/// Module providing Unicode-to-ASCII character mapping.
pub mod unicode_map;

pub use detector::detect_confusables;
pub use score::confusable_score;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cyrillic_homoglyph() {
        let input = "\u{0440}\u{0430}ypal";
        let flags = detect_confusables(input);
        assert!(flags.contains(&"VISUAL_MISMATCH_DETECTED".to_string()));
        assert!(flags.contains(&"CROSS_SCRIPT_MIXING".to_string()));
    }

    #[test]
    fn test_detect_f_hook() {
        let input = "\u{0192}unction";
        let flags = detect_confusables(input);
        assert!(flags.contains(&"VISUAL_MISMATCH_DETECTED".to_string()));
    }

    #[test]
    fn test_clean_input_no_flags() {
        let input = "hello";
        let flags = detect_confusables(input);
        assert!(flags.is_empty());
    }

    #[test]
    fn test_confusable_score_non_latin() {
        let input = "\u{0440}\u{0430}ypal";
        let score = confusable_score(input);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_confusable_score_clean() {
        let input = "hello world";
        let score = confusable_score(input);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_impersonation_detection() {
        let input = "\u{0440}\u{0430}ypal";
        let flags = detect_confusables(input);
        assert!(flags.contains(&"POTENTIAL_IMPERSONATION".to_string()));
    }
}
