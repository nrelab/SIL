use crate::unicode_map::to_ascii_equivalent;

#[must_use]
pub fn confusable_score(input: &str) -> f32 {
    let mut score: f32 = 0.0;

    if contains_multiple_scripts(input) {
        score += 0.4;
    }

    if has_similar_ascii_target(input) {
        score += 0.4;
    }

    if has_obfuscation_patterns(input) {
        score += 0.2;
    }

    score.min(1.0)
}

fn contains_multiple_scripts(input: &str) -> bool {
    let mut latin = false;
    let mut non_latin = false;

    for c in input.chars() {
        let code = c as u32;

        if (0x0041..=0x007A).contains(&code) {
            latin = true;
        } else if code > 0x007F {
            non_latin = true;
        }
    }

    latin && non_latin
}

fn has_similar_ascii_target(input: &str) -> bool {
    let normalized = to_ascii_equivalent(input);
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

    let lower = normalized.to_lowercase();
    targets.iter().any(|t| {
        let dist = levenshtein_distance(&lower, t);
        dist <= 2 && dist > 0
    })
}

fn has_obfuscation_patterns(input: &str) -> bool {
    let mut has_digit = false;
    let mut has_letter = false;

    for c in input.chars() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c.is_alphabetic() {
            has_letter = true;
        }
    }

    has_digit && has_letter
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let a_len = a.len();
    let b_len = b.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr: Vec<usize> = vec![0; b_len + 1];

    for (i, a_char) in a.iter().enumerate().take(a_len) {
        curr[0] = i + 1;
        for (j, b_char) in b.iter().enumerate().take(b_len) {
            let cost = usize::from(a_char != b_char);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        (prev, curr) = (curr, prev);
    }

    prev[b_len]
}
