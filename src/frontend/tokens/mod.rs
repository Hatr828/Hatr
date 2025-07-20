mod keyword;
mod literal;
mod operator;
mod delimiter;

pub use keyword::Keyword;
pub use literal::LiteralKind;
pub use operator::Operator;
pub use delimiter::Delimiter;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Literal(LiteralKind),
    Operator(Operator),
    Delimiter(Delimiter),
    NotFound,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
