use crate::frontend::lexer::{self, Lexer};
use crate::frontend::tokens::{Token, TokenKind, Span, Keyword, LiteralKind, Operator, Delimiter};
use crate::frontend::act::{BinOp, Expr, Stmt, Program};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let first = lexer.next_token();
        Parser { lexer, current: first }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current.kind != TokenKind::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current.kind {
            TokenKind::Keyword(Keyword::Println) => {
                self.advance(); // Consume 'println'
                let expr = self.parse_expression();

                if self.current.kind != TokenKind::Delimiter(Delimiter::Semicolon) {
                    panic!("Expected ';' after println statement, found: {:?}", self.current);
                }
                self.advance(); 

                Stmt::Println(expr)
            }
            TokenKind::Keyword(Keyword::Int) => {
                self.advance(); 
                
                 if let TokenKind::Identifier(name) = &self.current.kind {
                    let ident = name.clone();
                    self.advance();

                    if self.current.kind == TokenKind::Operator(Operator::Assign) {
                        self.advance();

                        let expr = self.parse_expression();

                        if self.current.kind != TokenKind::Delimiter(Delimiter::Semicolon) {
                            panic!("Expected ';' after println statement, found");
                        }

                        self.advance(); 

                        Stmt::Declaration(ident, expr)

                    } else {
                        panic!("Expected '=' after declaration identifier {:?}", self.current.kind);
                    }
                } else {
                    panic!("Expected identifier after declaration");
                }
            }
            _ => {
                let expr = self.parse_expression();
                Stmt::Expr(expr)
            }
        }
    }

    /// Parse expressions with + and -
    fn parse_expression(&mut self) -> Expr {
        let mut node = self.parse_term();

        while matches!(self.current.kind, TokenKind::Operator(Operator::Plus) | TokenKind::Operator(Operator::Minus)) {
            let op_kind = if let TokenKind::Operator(op) = self.current.kind { op } else { unreachable!() };
            self.advance();
            let rhs = self.parse_term();
            node = Expr::Binary {
                op: match op_kind {
                    Operator::Plus => BinOp::Add,
                    Operator::Minus => BinOp::Sub,
                    _ => unreachable!(),
                },
                left: Box::new(node),
                right: Box::new(rhs),
            };
        }
        node
    }

    /// Parse multiplication/division
    fn parse_term(&mut self) -> Expr {
       let mut node = self.parse_factor();
       while matches!(self.current.kind, TokenKind::Operator(Operator::Star) | TokenKind::Operator(Operator::Slash)) {
            let op_kind = if let TokenKind::Operator(op) = self.current.kind { op } else { unreachable!() };
            self.advance();
            let rhs = self.parse_factor();
            node = Expr::Binary {
                op: match op_kind {
                    Operator::Star => BinOp::Mul,
                    Operator::Slash => BinOp::Div,
                    _ => unreachable!(),
                },
                left: Box::new(node),
                right: Box::new(rhs),
            };
        }
        node
    }
    
    /// Parse integer, variablе
    fn parse_factor(&mut self) -> Expr {
      let node = match &self.current.kind {
            TokenKind::Literal(LiteralKind::Int(value)) => {
                let n = *value;
                self.advance(); 
                Expr::Int(n)
            }
            TokenKind::Literal(LiteralKind::Str(s)) => {
                let s_clone = s.clone();
                self.advance();
                Expr::Var(s_clone)
            }
            TokenKind::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Expr::Var(id)
            }
            TokenKind::Delimiter(Delimiter::LParen) => { // '('
                self.advance(); // Consume '('
                let expr = self.parse_expression();

                if self.current.kind != TokenKind::Delimiter(Delimiter::RParen) {
                    panic!("Expected ')' after expression, found: {:?}", self.current);
                }
                self.advance(); 

                expr
            }
            _ => panic!("Unexpected token in factor: {:?}", self.current),
        };

        node
    }
}