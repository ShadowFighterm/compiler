use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    BoolLit(bool),
    Identifier(String),
    UnaryOp { op: TokenKind, expr: Box<Expr> },
    BinaryOp { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: TokenKind,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    FnDecl {
        ty: TokenKind,        // Examples: TokenKind::T_INT
        name: String,
        params: Vec<Param>,
        block: Vec<Stmt>,
    },
    VarDecl {
        ty: TokenKind,
        name: String,
        expr: Option<Expr>,
    },
    Ret {
        expr: Option<Expr>,
    },
    For {
        init: Box<Stmt>,
        cond: Expr,
        updt: Box<Stmt>,
        block: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    Break,
    ExprStmt {
        expr: Expr,
    },
}
