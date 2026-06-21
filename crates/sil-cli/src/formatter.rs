use sil_policy::Decision;

/// Formats a [`Decision`] into a human-readable string with emoji indicators.
///
/// - `Allow` → 🟢 SAFE
/// - `Warn` → 🟡 WARNING
/// - `Block` → 🔴 BLOCKED
/// - `Rewrite(s)` → 🔧 REWRITE → {s}
pub fn format_decision(decision: &Decision) -> String {
    match decision {
        Decision::Allow => "\u{1F7E2} SAFE".to_string(),
        Decision::Warn => "\u{1F7E1} WARNING".to_string(),
        Decision::Block => "\u{1F534} BLOCKED".to_string(),
        Decision::Rewrite(s) => format!("\u{1F527} REWRITE \u{2192} {s}"),
    }
}
