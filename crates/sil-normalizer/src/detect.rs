/// Scans input text for suspicious Unicode patterns.
///
/// Returns a list of issue flag strings. Currently detects:
/// - `ZERO_WIDTH_CHARACTER_DETECTED` — zero-width space, non-joiner, joiner, or BOM
/// - `SUSPICIOUS_UNICODE_PATTERN` — Greek (0x0370–0x03FF) or Cyrillic (0x0400–0x04FF) characters
///
/// Returns an empty `Vec` if no issues are found.
#[must_use]
pub fn scan_input(input: &str) -> Vec<String> {
    let mut issues = vec![];

    if contains_zero_width(input) {
        issues.push("ZERO_WIDTH_CHARACTER_DETECTED".to_string());
    }

    if contains_suspicious_unicode(input) {
        issues.push("SUSPICIOUS_UNICODE_PATTERN".to_string());
    }

    issues
}

fn contains_zero_width(input: &str) -> bool {
    input
        .chars()
        .any(|c| matches!(c, '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}'))
}

fn contains_suspicious_unicode(input: &str) -> bool {
    input.chars().any(|c| {
        let code = c as u32;
        (0x0370..=0x03FF).contains(&code) || // Greek
        (0x0400..=0x04FF).contains(&code) // Cyrillic
    })
}
