#![warn(clippy::pedantic)]
#![doc = "Unicode normalization engine for the Semantic Integrity Layer."]
#![doc = ""]
#![doc = "Provides NFKC normalization, zero-width character filtering,"]
#![doc = "and detection of suspicious Unicode patterns (Greek, Cyrillic)"]
#![doc = "in input text."]

/// Module for detecting suspicious Unicode patterns in text.
pub mod detect;
/// Module for normalizing Unicode input via NFKC and invisible character removal.
pub mod normalize;

pub use detect::scan_input;
pub use normalize::normalize_input;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_removes_zero_width() {
        let input = "us\u{200B}er";
        assert_eq!(normalize_input(input), "user");
    }

    #[test]
    fn test_scan_detects_zero_width() {
        let input = "us\u{200B}er";
        let issues = scan_input(input);
        assert!(issues.contains(&"ZERO_WIDTH_CHARACTER_DETECTED".to_string()));
    }

    #[test]
    fn test_scan_detects_suspicious_unicode() {
        let input = "\u{0440}\u{0430}ypal";
        let issues = scan_input(input);
        assert!(issues.contains(&"SUSPICIOUS_UNICODE_PATTERN".to_string()));
    }

    #[test]
    fn test_clean_input_no_issues() {
        let input = "hello world";
        let clean = normalize_input(input);
        let issues = scan_input(input);
        assert_eq!(clean, "hello world");
        assert!(issues.is_empty());
    }
}
