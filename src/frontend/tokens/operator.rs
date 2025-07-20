#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,    // '+'
    Minus,   // '-'
    Star,    // '*'
    Slash,   // '/'
    Eq,      // '=='
    Neq,     // '!='
    Lt, Gt,  // '<', '>'
}
