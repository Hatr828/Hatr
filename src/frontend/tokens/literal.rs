#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Int(i64),
    Str(String),
    Bool(bool),
}
