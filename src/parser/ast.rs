use crate::token::TokenKind;
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    StringLit(String),
    Binary {
        left: Box<Expr>, //Box is a smart pointer that allocates data on heap, here particularly pointer to another expression on the heap
        operator: TokenKind,
        right: Box<Expr>,
    },
    Unary {
        operator: TokenKind,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Grouping(Box<Expr>), // just a wrapper around another expression
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        type_annot: Option<TokenKind>, // T_INT, T_FLOAT, etc.
        value: Expr,
    },
    Block(Vec<Stmt>),
    Return(Option<Expr>),
    Break,
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        params: Vec<Param>,
        return_type: Option<TokenKind>,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: TokenKind, // T_INT, T_FLOAT, etc.
}

#[derive(Debug, Clone)]
pub enum Decl {
    Function {
        name: String,
        params: Vec<Param>,
        return_type: Option<TokenKind>,
        body: Box<Stmt>,
    },
    GlobalVar {
        name: String,
        type_annot: Option<TokenKind>,
        value: Option<Expr>,
    },
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Decl>,
}