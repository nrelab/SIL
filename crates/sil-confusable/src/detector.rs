use crate::unicode_map::to_ascii_equivalent;

pub fn detect_confusables(input: &str) -> Vec<String> {
    let mut flags = vec![];

    let normalized = to_ascii_equivalent(input);

    if normalized != input {
        flags.push("VISUAL_MISMATCH_DETECTED".to_string());
    }

    if has_cross_script_mix(input) {
        flags.push("CROSS_SCRIPT_MIXING".to_string());
    }

    if looks_like_common_target(input) {
        flags.push("POTENTIAL_IMPERSONATION".to_string());
    }

    flags
}

fn has_cross_script_mix(input: &str) -> bool {
    let mut has_latin = false;
    let mut has_cyrillic = false;
    let mut has_greek = false;

    for c in input.chars() {
        let code = c as u32;
        match code {
            0x0041..=0x005A | 0x0061..=0x007A => has_latin = true,
            0x0400..=0x04FF => has_cyrillic = true,
            0x0370..=0x03FF => has_greek = true,
            _ => {}
        }
    }

    (has_latin && has_cyrillic) || (has_latin && has_greek) || (has_cyrillic && has_greek)
}

fn looks_like_common_target(input: &str) -> bool {
    let normalized = to_ascii_equivalent(input).to_lowercase();
    let targets = [
        "paypal",
        "google",
        "facebook",
        "amazon",
        "apple",
        "microsoft",
        "netflix",
        "github",
        "gmail",
        "whatsapp",
    ];
    targets.contains(&normalized.as_str())
}
