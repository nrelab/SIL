#[derive(Debug, PartialEq)]
pub enum Decision {
    Allow,
    Warn,
    Block,
    Rewrite(String),
}
