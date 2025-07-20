#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    LParen,    // '('
    RParen,    // ')'
    LBrace,    // '{'
    RBrace,    // '}'
    Semicolon, // ';'
    Comma,    // ','
}
