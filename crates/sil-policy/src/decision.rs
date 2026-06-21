/// Represents the decision outcome of the policy evaluation engine.
///
/// # Variants
/// - `Allow` — input is safe and permitted
/// - `Warn` — input has moderate risk; flagged for review
/// - `Block` — input has high risk and is rejected
/// - `Rewrite(String)` — input has repairable issues; the contained string is the sanitized version
#[derive(Debug, PartialEq)]
pub enum Decision {
    Allow,
    Warn,
    Block,
    Rewrite(String),
}
