use unicode_normalization::UnicodeNormalization;

#[must_use]
pub fn normalize_input(input: &str) -> String {
    let nfkc: String = input.nfkc().collect();

    nfkc.chars().filter(|c| !is_invisible(*c)).collect()
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
