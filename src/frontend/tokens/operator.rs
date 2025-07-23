#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,    // '+'
    Minus,   // '-'
    Star,    // '*'
    Slash,   // '/'
    Assign, // '='
    Eq,      // '=='
    Neq,     // '!='
    Lt, Gt,  // '<', '>'
}
