use crate::error::NormalizerError;
use unicode_normalization::UnicodeNormalization;

/// Normalizes input text using NFKC normalization and strips invisible characters.
///
/// Removes the following characters:
/// - `\u{200B}` — zero-width space
/// - `\u{200C}` — zero-width non-joiner
/// - `\u{200D}` — zero-width joiner
/// - `\u{FEFF}` — byte order mark (BOM)
///
/// # Errors
///
/// Returns [`NormalizerError::EmptyInput`] if `input` is empty.
pub fn normalize_input(input: &str) -> Result<String, NormalizerError> {
    if input.is_empty() {
        return Err(NormalizerError::EmptyInput);
    }
    let nfkc: String = input.nfkc().collect();

    Ok(nfkc.chars().filter(|c| !is_invisible(*c)).collect())
}

fn is_invisible(c: char) -> bool {
    matches!(
        c,
        '\u{200B}' | // zero-width space
        '\u{200C}' | // zero-width non-joiner
        '\u{200D}' | // zero-width joiner
        '\u{FEFF}' // BOM
    )
}
