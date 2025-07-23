#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Println(Expr),

    Declaration(String, Expr),
  
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),

    Var(String),

    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
