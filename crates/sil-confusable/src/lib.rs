#![warn(clippy::pedantic)]

pub mod detector;
pub mod score;
pub mod unicode_map;

pub use detector::detect_confusables;
pub use score::confusable_score;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cyrillic_homoglyph() {
        let input = "раypal";
        let flags = detect_confusables(input);
        assert!(flags.contains(&"VISUAL_MISMATCH_DETECTED".to_string()));
        assert!(flags.contains(&"CROSS_SCRIPT_MIXING".to_string()));
    }

    #[test]
    fn test_detect_f_hook() {
        let input = "ƒunction";
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
        let input = "раypal";
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
        let input = "раypal";
        let flags = detect_confusables(input);
        assert!(flags.contains(&"POTENTIAL_IMPERSONATION".to_string()));
    }
}
