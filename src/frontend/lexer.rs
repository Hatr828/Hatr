use std::str::Chars;
use std::iter::Peekable;
use crate::frontend::tokens::{Token, TokenKind, Span, Keyword, LiteralKind, Operator, Delimiter};


pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let first = chars.next();

        Lexer {
            input: chars,
            current: first,
            line: 1,
            column: 0,
        }
    }

    fn read_char(&mut self) {
       self.current = self.input.next();
        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while self.current.map_or(false, |c| c.is_whitespace()) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let span = Span { line: self.line, column: self.column };

        let kind = match self.current {
            Some(c) if is_identifier_start(c) => return self.read_identifier_or_keyword(span),
            Some(c) if c.is_digit(10) => return self.read_number(span),
            Some('"') => return self.read_string(span),

            Some(c) => {
                let tk = match c {
                    '+' => TokenKind::Operator(Operator::Plus),
                    '-' => TokenKind::Operator(Operator::Minus),
                    '*' => TokenKind::Operator(Operator::Star),
                    '/' => TokenKind::Operator(Operator::Slash),
                    '<' => TokenKind::Operator(Operator::Lt),
                    '>' => TokenKind::Operator(Operator::Gt),
                    '(' => TokenKind::Delimiter(Delimiter::LParen),
                    ')' => TokenKind::Delimiter(Delimiter::RParen),
                    '{' => TokenKind::Delimiter(Delimiter::LBrace),
                    '}' => TokenKind::Delimiter(Delimiter::RBrace),
                    ';' => TokenKind::Delimiter(Delimiter::Semicolon),
                    ',' => TokenKind::Delimiter(Delimiter::Comma),
                    '=' => TokenKind::Operator(Operator::Eq),
                    _   => TokenKind::NotFound,
                };
                self.read_char();
                tk
            }
            None => TokenKind::Eof,
        };

        Token { kind, span }
    }

    fn read_identifier_or_keyword(&mut self, start: Span) -> Token {
        let mut identifier = String::new();
        
        while self.current.map_or(false, |c| is_identifier_part(c)) {
            if let Some(c) = self.current {
                identifier.push(c);
            }
            self.read_char();
        }

        let kind = match identifier.as_str() {
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "while" => TokenKind::Keyword(Keyword::While),
            "for" => TokenKind::Keyword(Keyword::For),
            "println" => TokenKind::Keyword(Keyword::Println),
            "int" => TokenKind::Keyword(Keyword::Int),
            _ => TokenKind::Identifier(identifier),
        };

        Token { kind, span: start }
    }

    fn read_number(&mut self, start: Span) -> Token {
        let mut num = String::new();

        while self.current.map_or(false, |c| c.is_ascii_digit()) {
            if let Some(c) = self.current { num.push(c) }
            self.read_char();
        }

        let value = num.parse::<i64>().unwrap_or(0);
        Token { kind: TokenKind::Literal(LiteralKind::Int(value)), span: start }
    }

    fn read_string(&mut self, start: Span) -> Token {
        self.read_char();
        let mut s = String::new();

        while let Some(c) = self.current {
            if c == '"' { break; }
            s.push(c);
            self.read_char();
        }

        self.read_char(); 
        Token { kind: TokenKind::Literal(LiteralKind::Str(s)), span: start }
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}


