use crate::token::{Token, TokenKind};
use crate::ast::*;
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    FailedToFindToken(TokenKind),
    ExpectedTypeToken,
    ExpectedIdentifier,
    UnexpectedToken(TokenKind),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> { 
        self.tokens.get(self.pos) 
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        match self.peek() {
            Some(t) if t.kind == kind => { 
                self.next();
                Ok(()) 
            }
            Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while self.pos < self.tokens.len() {
            stmts.push(self.parse_declaration()?);
        }
        Ok(stmts)
    }

    fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Some(t) if t.kind == TokenKind::T_FUNCTION => self.parse_fn_decl(),
            Some(t) if matches!(t.kind, TokenKind::T_INT | TokenKind::T_FLOAT | TokenKind::T_BOOL | TokenKind::T_STRING) => self.parse_var_decl(),
            _ => self.parse_statement(),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<Stmt, ParseError> {
        self.match_kind(TokenKind::T_FUNCTION)?;
        let ret_type = self.parse_type()?;
        let name = self.consume_identifier()?;
        self.match_kind(TokenKind::T_PARENL)?;

        let mut params = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::T_PARENR {
                break;
            }
            let ty = self.parse_type()?;
            let pname = self.consume_identifier()?;
            params.push(Param { ty, name: pname });
            if let Some(tok) = self.peek() {
                if tok.kind == TokenKind::T_COMMA {
                    self.next();
                }
            }
        }
        self.match_kind(TokenKind::T_PARENR)?;
        self.match_kind(TokenKind::T_BRACEL)?;
        let block = self.parse_block(TokenKind::T_BRACER)?;
        self.match_kind(TokenKind::T_BRACER)?;
        self.match_kind(TokenKind::T_DOT)?;

        Ok(Stmt::FnDecl { ty: ret_type, name, params, block })
    }

    fn consume_identifier(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(Token { kind: TokenKind::T_IDENTIFIER(name), .. }) => Ok(name.clone()),
            Some(_) => Err(ParseError::ExpectedIdentifier),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        let ty = self.parse_type()?;
        let name = self.consume_identifier()?;
        let expr = if let Some(tok) = self.peek() {
            if tok.kind == TokenKind::T_ASSIGNOP {
                self.next();
                Some(self.parse_expr()?)
            } else { None }
        } else { None };
        self.match_kind(TokenKind::T_DOT)?;
        Ok(Stmt::VarDecl { ty, name, expr })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::T_RETURN) => { 
                self.next(); 
                let expr = Some(self.parse_expr()?); 
                self.match_kind(TokenKind::T_DOT)?; 
                Ok(Stmt::Ret { expr }) 
            }
            Some(TokenKind::T_BREAK) => { 
                self.next(); 
                Ok(Stmt::Break) 
            }
            Some(TokenKind::T_IF) => self.parse_if(),
            Some(TokenKind::T_FOR) => self.parse_for(),
            _ => { 
                let expr = self.parse_expr()?; 
                self.match_kind(TokenKind::T_DOT)?; 
                Ok(Stmt::ExprStmt { expr }) 
            }
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.match_kind(TokenKind::T_IF)?;
        self.match_kind(TokenKind::T_PARENL)?;
        let cond = self.parse_expr()?;
        self.match_kind(TokenKind::T_PARENR)?;
        self.match_kind(TokenKind::T_BRACEL)?;
        let then_block = self.parse_block(TokenKind::T_BRACER)?;
        self.match_kind(TokenKind::T_BRACER)?;
        let else_block = if let Some(Token { kind: TokenKind::T_ELSE, .. }) = self.peek() {
            self.next();
            self.match_kind(TokenKind::T_BRACEL)?;
            let block = self.parse_block(TokenKind::T_BRACER)?;
            self.match_kind(TokenKind::T_BRACER)?;
            block
        } else { Vec::new() };
        Ok(Stmt::If { cond, then_block, else_block })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.match_kind(TokenKind::T_FOR)?;
        self.match_kind(TokenKind::T_PARENL)?;
        let init = Box::new(self.parse_var_decl()?);
        let cond = self.parse_expr()?;
        self.match_kind(TokenKind::T_DOT)?;
        let updt = Box::new(self.parse_statement()?);
        self.match_kind(TokenKind::T_PARENR)?;
        self.match_kind(TokenKind::T_BRACEL)?;
        let block = self.parse_block(TokenKind::T_BRACER)?;
        self.match_kind(TokenKind::T_BRACER)?;
        Ok(Stmt::For { init, cond, updt, block })
    }

    fn parse_type(&mut self) -> Result<TokenKind, ParseError> {
        match self.peek() {
            Some(Token { kind: TokenKind::T_INT, .. }) |
            Some(Token { kind: TokenKind::T_FLOAT, .. }) |
            Some(Token { kind: TokenKind::T_BOOL, .. }) |
            Some(Token { kind: TokenKind::T_STRING, .. }) => {
                let kind = self.peek().unwrap().kind.clone();
                self.next();
                Ok(kind)
            }
            Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_expr(0)
    }
    
    fn parse_binary_expr(&mut self, _min_prec: u8) -> Result<Expr, ParseError> {
        // For simplicity, just parse primary for now
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().cloned();

        match tok {
            Some(tok) => {
                match tok.kind {
                    TokenKind::T_INTLIT(i) => {
                        self.next();
                        Ok(Expr::IntLit(i))
                    }
                    TokenKind::T_IDENTIFIER(s) => {
                        self.next();
                        Ok(Expr::Identifier(s))
                    }
                    _ => Err(ParseError::ExpectedExpr),
                }
            }
            None => Err(ParseError::ExpectedExpr),
        }
    }

    fn parse_block(&mut self, end: TokenKind) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.kind == end { break; }
            stmts.push(self.parse_declaration()?);
        }
        Ok(stmts)
    }
}



// use crate::token::{Token, TokenKind};
// use crate::ast::*;
// #[derive(Debug)]
// pub enum ParseError {
//     UnexpectedEOF,
//     FailedToFindToken(TokenKind),
//     ExpectedTypeToken,
//     ExpectedIdentifier,
//     UnexpectedToken(TokenKind),
//     ExpectedFloatLit,
//     ExpectedIntLit,
//     ExpectedStringLit,
//     ExpectedBoolLit,
//     ExpectedExpr,
// }

// pub struct Parser {
//     tokens: Vec<Token>,
//     pos: usize,
// }

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Parser { tokens, pos: 0 }
//     }

//     fn peek(&self) -> Option<&Token> { 
//         self.tokens.get(self.pos) 
//     }

//     fn next(&mut self) -> Option<Token> {
//         if self.pos < self.tokens.len() {
//             let tok = self.tokens[self.pos].clone();
//             self.pos += 1;
//             Some(tok)
//         } else {
//             None
//         }
//     }

//     fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParseError> {
//         match self.peek() {
//             Some(t) if t.kind == kind => { 
//                 self.next();
//                 Ok(()) 
//             }
//             Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
//         let mut stmts = Vec::new();
//         while self.pos < self.tokens.len() {
//             stmts.push(self.parse_declaration()?);
//         }
//         Ok(stmts)
//     }

//     fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
//         match self.peek() {
//             Some(t) if t.kind == TokenKind::T_FUNCTION => self.parse_fn_decl(),
//             Some(t) if matches!(t.kind, TokenKind::T_INT | TokenKind::T_FLOAT | TokenKind::T_BOOL | TokenKind::T_STRING) => self.parse_var_decl(),
//             _ => self.parse_statement(),
//         }
//     }

//     fn parse_fn_decl(&mut self) -> Result<Stmt, ParseError> {
//         self.match_kind(TokenKind::T_FUNCTION)?;
//         let ret_type = self.parse_type()?;
//         let name = self.consume_identifier()?;
//         self.match_kind(TokenKind::T_PARENL)?;

//         let mut params = Vec::new();
//         while let Some(tok) = self.peek() {
//             if tok.kind == TokenKind::T_PARENR {
//                 break;
//             }
//             let ty = self.parse_type()?;
//             let pname = self.consume_identifier()?;
//             params.push(Param { ty, name: pname });
//             if let Some(tok) = self.peek() {
//                 if tok.kind == TokenKind::T_COMMA {
//                     self.next();
//                 }
//             }
//         }
//         self.match_kind(TokenKind::T_PARENR)?;
//         self.match_kind(TokenKind::T_BRACEL)?;
//         let block = self.parse_block(TokenKind::T_BRACER)?;
//         self.match_kind(TokenKind::T_BRACER)?;
//         self.match_kind(TokenKind::T_DOT)?;

//         Ok(Stmt::FnDecl { ty: ret_type, name, params, block })
//     }

//     fn consume_identifier(&mut self) -> Result<String, ParseError> {
//         match self.next() {
//             Some(Token { kind: TokenKind::T_IDENTIFIER(name), .. }) => Ok(name.clone()),
//             Some(_) => Err(ParseError::ExpectedIdentifier),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
//         let ty = self.parse_type()?;
//         let name = self.consume_identifier()?;
//         let expr = if let Some(tok) = self.peek() {
//             if tok.kind == TokenKind::T_ASSIGNOP {
//                 self.next();
//                 Some(self.parse_expr()?)
//             } else { None }
//         } else { None };
//         self.match_kind(TokenKind::T_DOT)?;
//         Ok(Stmt::VarDecl { ty, name, expr })
//     }

//     fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
//         match self.peek().map(|t| &t.kind) {
//             Some(TokenKind::T_RETURN) => { 
//                 self.next(); 
//                 let expr = Some(self.parse_expr()?); 
//                 self.match_kind(TokenKind::T_DOT)?; 
//                 Ok(Stmt::Ret { expr }) 
//             }
//             Some(TokenKind::T_BREAK) => { 
//                 self.next(); 
//                 Ok(Stmt::Break) 
//             }
//             Some(TokenKind::T_IF) => self.parse_if(),
//             Some(TokenKind::T_FOR) => self.parse_for(),
//             _ => { 
//                 let expr = self.parse_expr()?; 
//                 self.match_kind(TokenKind::T_DOT)?; 
//                 Ok(Stmt::ExprStmt { expr }) 
//             }
//         }
//     }

//     fn parse_if(&mut self) -> Result<Stmt, ParseError> {
//         self.match_kind(TokenKind::T_IF)?;
//         self.match_kind(TokenKind::T_PARENL)?;
//         let cond = self.parse_expr()?;
//         self.match_kind(TokenKind::T_PARENR)?;
//         self.match_kind(TokenKind::T_BRACEL)?;
//         let then_block = self.parse_block(TokenKind::T_BRACER)?;
//         self.match_kind(TokenKind::T_BRACER)?;
//         let else_block = if let Some(Token { kind: TokenKind::T_ELSE, .. }) = self.peek() {
//             self.next();
//             self.match_kind(TokenKind::T_BRACEL)?;
//             let block = self.parse_block(TokenKind::T_BRACER)?;
//             self.match_kind(TokenKind::T_BRACER)?;
//             block
//         } else { Vec::new() };
//         Ok(Stmt::If { cond, then_block, else_block })
//     }

//     fn parse_for(&mut self) -> Result<Stmt, ParseError> {
//         self.match_kind(TokenKind::T_FOR)?;
//         self.match_kind(TokenKind::T_PARENL)?;
//         let init = Box::new(self.parse_var_decl()?);
//         let cond = self.parse_expr()?;
//         self.match_kind(TokenKind::T_DOT)?;
//         let updt = Box::new(self.parse_statement()?);
//         self.match_kind(TokenKind::T_PARENR)?;
//         self.match_kind(TokenKind::T_BRACEL)?;
//         let block = self.parse_block(TokenKind::T_BRACER)?;
//         self.match_kind(TokenKind::T_BRACER)?;
//         Ok(Stmt::For { init, cond, updt, block })
//     }

//     fn parse_type(&mut self) -> Result<TokenKind, ParseError> {
//         match self.peek() {
//             Some(Token { kind: TokenKind::T_INT, .. }) |
//             Some(Token { kind: TokenKind::T_FLOAT, .. }) |
//             Some(Token { kind: TokenKind::T_BOOL, .. }) |
//             Some(Token { kind: TokenKind::T_STRING, .. }) => {
//                 let kind = self.peek().unwrap().kind.clone();
//                 self.next();
//                 Ok(kind)
//             }
//             Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn parse_expr(&mut self) -> Result<Expr, ParseError> {
//         self.parse_binary_expr(0)
//     }
    
//     fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
//         let mut left = self.parse_primary_expr()?;
//         // TODO: implement operator precedence parsing here
//         Ok(left)
//     }

//     fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
//         let tok = self.peek().cloned();

//         match tok {
//             Some(tok) => {
//                 match tok.kind {
//                     TokenKind::T_INTLIT(i) => {
//                         self.next();
//                         Ok(Expr::IntLit(i))
//                     }
//                     TokenKind::T_IDENTIFIER(s) => {
//                         self.next();
//                         Ok(Expr::Identifier(s))
//                     }
//                     _ => Err(ParseError::ExpectedExpr),
//                 }
//             }
//             None => Err(ParseError::ExpectedExpr),
//         }
//     }

//     fn parse_block(&mut self, end: TokenKind) -> Result<Vec<Stmt>, ParseError> {
//         let mut stmts = Vec::new();
//         while let Some(tok) = self.peek() {
//             if tok.kind == end { break; }
//             stmts.push(self.parse_declaration()?);
//         }
//         Ok(stmts)
//     }
// }



// // use crate::token::{Token, TokenKind};
// // use crate::ast::*;
// // #[derive(Debug)]
// // pub enum ParseError {
// //     UnexpectedEOF,
// //     FailedToFindToken(TokenKind),
// //     ExpectedTypeToken,
// //     ExpectedIdentifier,
// //     UnexpectedToken(TokenKind),
// //     ExpectedFloatLit,
// //     ExpectedIntLit,
// //     ExpectedStringLit,
// //     ExpectedBoolLit,
// //     ExpectedExpr,
// // }

// // pub struct Parser {
// //     tokens: Vec<Token>,
// //     pos: usize,
// // }

// // impl Parser {
// //     pub fn new(tokens: Vec<Token>) -> Self {
// //         Parser { tokens, pos: 0 }
// //     }

// //     fn peek(&self) -> Option<&Token> { 
// //         self.tokens.get(self.pos) 
// //     }

// //     fn next(&mut self) -> Option<Token> {
// //         if self.pos < self.tokens.len() {
// //             let tok = self.tokens[self.pos].clone();
// //             self.pos += 1;
// //             Some(tok)
// //         } else {
// //             None
// //         }
// //     }

// //     fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParseError> {
// //         match self.peek() {
// //             Some(t) if t.kind == kind => { 
// //                 self.next();
// //                 Ok(()) 
// //             }
// //             Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
// //             None => Err(ParseError::UnexpectedEOF),
// //         }
// //     }

// //     pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
// //         let mut stmts = Vec::new();
// //         while self.pos < self.tokens.len() {
// //             stmts.push(self.parse_declaration()?);
// //         }
// //         Ok(stmts)
// //     }

// //     fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
// //         match self.peek() {
// //             Some(t) if t.kind == TokenKind::T_FUNCTION => self.parse_fn_decl(),
// //             Some(t) if t.kind == TokenKind::T_INT => self.parse_var_decl(),
// //             _ => self.parse_statement(),
// //         }
// //     }


// //     fn parse_fn_decl(&mut self) -> Result<Stmt, ParseError> {
// //         self.match_kind(TokenKind::T_FUNCTION)?;
// //         //self.match_kind(TokenKind::T_INT)?;
// //         self.parse_type()?;  // Use a helper to parse type tokens
// //         let name = self.consume_identifier()?;
// //         self.match_kind(TokenKind::T_PARENL)?;

// //         let mut params = Vec::new();
// //         while let Some(tok) = self.peek() {
// //             if tok.kind != TokenKind::T_PARENR {
// //                 self.parse_type()?;
// //                 let pname = self.consume_identifier()?;
// //                 params.push(Param { ty: TokenKind::T_INT, name: pname });
// //                 if let Some(tok) = self.peek() {
// //                     if tok.kind == TokenKind::T_COMMA { self.next(); }
// //                 }
// //             } else { break; }
// //         }
// //         self.match_kind(TokenKind::T_PARENR)?;
// //         self.match_kind(TokenKind::T_BRACEL)?;
// //         let block = self.parse_block(TokenKind::T_BRACER)?;
// //         self.match_kind(TokenKind::T_BRACER)?;
// //         self.match_kind(TokenKind::T_DOT)?;

// //         Ok(Stmt::FnDecl { ty: TokenKind::T_INT, name, params, block })
// //     }

// //     fn consume_identifier(&mut self) -> Result<String, ParseError> {
// //         match self.next() {
// //             Some(Token { kind: TokenKind::T_IDENTIFIER(name), .. }) => Ok(name.clone()),
// //             Some(_) => Err(ParseError::ExpectedIdentifier),
// //             None => Err(ParseError::UnexpectedEOF),
// //         }
// //     }

// //     fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
// //         self.parse_type()?;
// //         let name = self.consume_identifier()?;
// //         let expr = if let Some(tok) = self.peek() {
// //             if tok.kind == TokenKind::T_ASSIGNOP {
// //                 self.next();
// //                 Some(self.parse_expr()?)
// //             } else { None }
// //         } else { None };
// //         self.match_kind(TokenKind::T_DOT)?;
// //         Ok(Stmt::VarDecl { ty: TokenKind::T_INT, name, expr })
// //     }

// //     fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
// //         match self.peek().map(|t| &t.kind) {
// //             Some(TokenKind::T_RETURN) => { 
// //                 self.next(); 
// //                 let expr = Some(self.parse_expr()?); 
// //                 self.match_kind(TokenKind::T_DOT)?; 
// //                 Ok(Stmt::Ret { expr }) 
// //             }
// //             Some(TokenKind::T_BREAK) => { 
// //                 self.next(); 
// //                 Ok(Stmt::Break) 
// //             }
// //             Some(TokenKind::T_IF) => self.parse_if(),
// //             Some(TokenKind::T_FOR) => self.parse_for(),
// //             _ => { 
// //                 let expr = self.parse_expr()?; 
// //                 self.match_kind(TokenKind::T_DOT)?; 
// //                 Ok(Stmt::ExprStmt { expr }) 
// //             }
// //         }
// //     }

// //     fn parse_if(&mut self) -> Result<Stmt, ParseError> {
// //         self.match_kind(TokenKind::T_IF)?;
// //         self.match_kind(TokenKind::T_PARENL)?;
// //         let cond = self.parse_expr()?;
// //         self.match_kind(TokenKind::T_PARENR)?;
// //         self.match_kind(TokenKind::T_BRACEL)?;
// //         let then_block = self.parse_block(TokenKind::T_BRACER)?;
// //         self.match_kind(TokenKind::T_BRACER)?;
// //         let else_block = if let Some(Token { kind: TokenKind::T_ELSE, .. }) = self.peek() {
// //             self.next();
// //             self.match_kind(TokenKind::T_BRACEL)?;
// //             let block = self.parse_block(TokenKind::T_BRACER)?;
// //             self.match_kind(TokenKind::T_BRACER)?;
// //             block
// //         } else { Vec::new() };
// //         Ok(Stmt::If { cond, then_block, else_block })
// //     }

// //     fn parse_for(&mut self) -> Result<Stmt, ParseError> {
// //         self.match_kind(TokenKind::T_FOR)?;
// //         self.match_kind(TokenKind::T_PARENL)?;
// //         let init = Box::new(self.parse_var_decl()?);
// //         let cond = self.parse_expr()?;
// //         self.match_kind(TokenKind::T_DOT)?;
// //         let updt = Box::new(self.parse_statement()?);
// //         self.match_kind(TokenKind::T_PARENR)?;
// //         self.match_kind(TokenKind::T_BRACEL)?;
// //         let block = self.parse_block(TokenKind::T_BRACER)?;
// //         self.match_kind(TokenKind::T_BRACER)?;
// //         Ok(Stmt::For { init, cond, updt, block })
// //     }

// //     fn parse_type(&mut self) -> Result<TokenKind, ParseError> {
// //         match self.peek() {
// //             Some(Token { kind: TokenKind::T_INT, .. }) |
// //             Some(Token { kind: TokenKind::T_FLOAT, .. }) |
// //             Some(Token { kind: TokenKind::T_BOOL, .. }) |
// //             Some(Token { kind: TokenKind::T_STRING, .. }) => {
// //                 let kind = self.peek().unwrap().kind.clone();
// //                 self.next();
// //                 Ok(kind)
// //             }
// //             Some(t) => Err(ParseError::UnexpectedToken(t.kind.clone())),
// //             None => Err(ParseError::UnexpectedEOF),
// //         }
// //     }




// //     fn parse_expr(&mut self) -> Result<Expr, ParseError> {
// //         self.parse_binary_expr(0)
// //     }
    
// //     fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
// //         let mut left = self.parse_primary_expr()?;
// //         // TODO: implement operator precedence parsing here
// //         Ok(left)
// //     }


// //   fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
// //     let tok = self.peek().cloned(); // clone token to release borrow

// //     match tok {
// //         Some(tok) => {
// //             match tok.kind {
// //                 TokenKind::T_INTLIT(i) => {
// //                     self.next();
// //                     Ok(Expr::IntLit(i))
// //                 }
// //                 TokenKind::T_IDENTIFIER(s) => {
// //                     self.next();
// //                     Ok(Expr::Identifier(s))
// //                 }
// //                 _ => Err(ParseError::ExpectedExpr),
// //             }
// //         }
// //         None => Err(ParseError::ExpectedExpr),
// //     }
// // }



// //     fn parse_block(&mut self, end: TokenKind) -> Result<Vec<Stmt>, ParseError> {
// //         let mut stmts = Vec::new();
// //         while let Some(tok) = self.peek() {
// //             if tok.kind == end { break; }
// //             stmts.push(self.parse_declaration()?);
// //         }
// //         Ok(stmts)
// //     }
// // }
