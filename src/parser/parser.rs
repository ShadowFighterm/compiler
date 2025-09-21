use crate::token::{Token, TokenKind};
use crate::parser::error::{ParseError, ParseErrorKind};
use crate::parser::ast::{Expr, Stmt, Decl, Param, Program};

pub struct Parser<'a> {
    tokens: &'a [Token],  //not Vec<Token> because we dont need to own the tokens, just borrow them
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    // UTILITY func stuff
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> { //previous, because after advancing we might want to know what the last token was
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        //on peeking if no next token (None), return true (or invoked) -- we are at the verge and matches is true if the token kind is T_EOF, |t| holds the token from Some(token). -- idiomatic feel 
        self.peek().map_or(true, |t| matches!(t.kind, TokenKind::T_EOF))
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
            self.previous() //return the consumed one
        } else {
            None
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().map(|t| &t.kind) == Some(kind)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    
    fn consume(&mut self, kind: &TokenKind, error_msg: &str) -> Result<(), ParseError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
            Err(ParseError {
                kind: ParseErrorKind::Expected(error_msg.to_string()),
                line,
                col,
            })
        }
    }

    // expression parsing 
    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_equality()?; //parse LHS first and equality has higher precedence than assignment
        //? means if error, return the error another idomatic fanciness
        if self.match_token(&TokenKind::T_ASSIGNOP) {
            let value = self.parse_assignment()?; // Right-associative
            if let Expr::Identifier(name) = expr {
                return Ok(Expr::Binary {
                    left: Box::new(Expr::Identifier(name)),
                    operator: TokenKind::T_ASSIGNOP,
                    right: Box::new(value),
                });
            }
            let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
            return Err(ParseError { kind: ParseErrorKind::Expected("variable name".to_string()), line, col });
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(&TokenKind::T_EQUALSOP) || self.match_token(&TokenKind::T_NEQ) {
            let operator = self.previous().unwrap().kind.clone();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while self.match_token(&TokenKind::T_GT) || self.match_token(&TokenKind::T_GTE) ||
              self.match_token(&TokenKind::T_LT) || self.match_token(&TokenKind::T_LTE) {
            let operator = self.previous().unwrap().kind.clone();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        while self.match_token(&TokenKind::T_PLUS) || self.match_token(&TokenKind::T_MINUS) {
            let operator = self.previous().unwrap().kind.clone();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        while self.match_token(&TokenKind::T_STAR) || self.match_token(&TokenKind::T_SLASH) ||
              self.match_token(&TokenKind::T_PERCENT) {
            let operator = self.previous().unwrap().kind.clone();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&TokenKind::T_MINUS) || self.match_token(&TokenKind::T_NOT) {
            let operator = self.previous().unwrap().kind.clone();
            let right = self.parse_unary()?;
            return Ok(Expr::Unary {
                operator,
                expr: Box::new(right),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::T_BOOLLIT(b) => {
                    let b = *b;
                    self.advance();
                    return Ok(Expr::Boolean(b));
                }
                TokenKind::T_INTLIT(n) => {
                    let n = *n;
                    self.advance();
                    return Ok(Expr::Integer(n));
                }
                TokenKind::T_FLOATLIT(n) => {
                    let n = *n;
                    self.advance();
                    return Ok(Expr::Float(n));
                }
                TokenKind::T_STRINGLIT(s) => {
                    let s = s.clone();
                    self.advance();
                    return Ok(Expr::StringLit(s));
                }
                TokenKind::T_IDENTIFIER(name) => {
                    let name = name.clone();
                    self.advance();

                    // Check if it's a function call
                    if self.match_token(&TokenKind::T_PARENL) {
                        return self.parse_call_expr(name);
                    }
                    return Ok(Expr::Identifier(name));
                }
                _ => {}
            }
        }

        if self.match_token(&TokenKind::T_PARENL) {
            let expr = self.parse_expression()?;
            self.consume(&TokenKind::T_PARENR, "')'")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let (line, col) = self.peek().map(|t| (t.line, t.col)).unwrap_or((0, 0));
        Err(ParseError { kind: ParseErrorKind::ExpectedExpr, line, col })
    }

    fn parse_call_expr(&mut self, callee: String) -> Result<Expr, ParseError> {
        let mut args = Vec::new();
        
        if !self.check(&TokenKind::T_PARENR) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(&TokenKind::T_COMMA) {
                    break;
                }
            }
        }
        
        self.consume(&TokenKind::T_PARENR, "')' after arguments")?;
        Ok(Expr::Call {
            callee: Box::new(Expr::Identifier(callee)),
            args,
        })
    }

    // statement parsing
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&TokenKind::T_RETURN) {
            return self.parse_return_statement();
        }
        if self.match_token(&TokenKind::T_IF) {
            return self.parse_if_statement();
        }
        if self.match_token(&TokenKind::T_WHILE) {
            return self.parse_while_statement();
        }
        if self.match_token(&TokenKind::T_FOR) {
            return self.parse_for_statement();
        }
        if self.match_token(&TokenKind::T_BRACEL) {
            return self.parse_block_statement();
        }
        
        // Variable declaration or expression statement
        if self.is_type_token(self.peek()) {
            self.parse_declaration_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = if !self.check(&TokenKind::T_SEMICOLON) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(&TokenKind::T_SEMICOLON, "';' after return value")?;
        Ok(Stmt::Return(value))
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenKind::T_PARENL, "'(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenKind::T_PARENR, "')' after condition")?;
        
        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_token(&TokenKind::T_ELSE) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenKind::T_PARENL, "'(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenKind::T_PARENR, "')' after condition")?;
        let body = Box::new(self.parse_statement()?);
        
        Ok(Stmt::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenKind::T_PARENL, "'(' after 'for'")?;
        
        // Initializer
        let init = if self.match_token(&TokenKind::T_SEMICOLON) {
            None
        } else if self.is_type_token(self.peek()) {
            Some(Box::new(self.parse_declaration_statement()?))
        } else {
            Some(Box::new(self.parse_expression_statement()?))
        };
        
        // Condition
        let condition = if !self.check(&TokenKind::T_SEMICOLON) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(&TokenKind::T_SEMICOLON, "';' after loop condition")?;
        
        // Increment
        let increment = if !self.check(&TokenKind::T_PARENR) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(&TokenKind::T_PARENR, "')' after for clauses")?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Stmt::For {
            init,
            condition,
            increment,
            body,
        })
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();
        
        while !self.check(&TokenKind::T_BRACER) && !self.is_at_end() {
            statements.push(self.parse_statement()?);  // Changed to parse_statement
        }
        
        self.consume(&TokenKind::T_BRACER, "'}' after block")?;
        Ok(Stmt::Block(statements))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression()?;
        self.consume(&TokenKind::T_SEMICOLON, "';' after expression")?;
        Ok(Stmt::Expr(expr))
    }
    
    fn parse_declaration_statement(&mut self) -> Result<Stmt, ParseError> {
        let type_annot = self.advance().map(|t| t.kind.clone()); // Consume type token
        let name = if let Some(token) = self.advance() {
            if let TokenKind::T_IDENTIFIER(n) = &token.kind {
                n.clone()
            } else {
                let (line, col) = (token.line, token.col);
                return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
            }
        } else {
            let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
            return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
        };
        
        let value = if self.match_token(&TokenKind::T_ASSIGNOP) {
            self.parse_expression()?
        } else {
            // Default values based on type
            match &type_annot {
                Some(TokenKind::T_INT) => Expr::Integer(0),
                Some(TokenKind::T_FLOAT) => Expr::Float(0.0),
                Some(TokenKind::T_BOOL) => Expr::Boolean(false),
                Some(TokenKind::T_STRING) => Expr::StringLit("".to_string()),
                _ => {
                    let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
                    return Err(ParseError { kind: ParseErrorKind::ExpectedTypeToken, line, col });
                }
            }
        };
        
        self.consume(&TokenKind::T_SEMICOLON, "';' after variable declaration")?;
        Ok(Stmt::Let {
            name,
            type_annot,
            value,
        })
    }

    // declaration parsing 
    fn parse_declaration(&mut self) -> Result<Decl, ParseError> {
        if self.match_token(&TokenKind::T_FUNCTION) {
            self.parse_function_declaration()
        } else if self.is_type_token(self.peek()) {
            self.parse_global_var_declaration()
        } else {
            // Treat as statement
            let stmt = self.parse_statement()?;
            Ok(Decl::GlobalVar {
                name: "".to_string(), // Placeholder
                type_annot: None,
                value: Some(match stmt {
                    Stmt::Expr(expr) => expr,
                    _ => {
                        let (line, col) = self.peek().map(|t| (t.line, t.col)).unwrap_or((0, 0));
                        return Err(ParseError { kind: ParseErrorKind::Expected("expression".to_string()), line, col });
                    }
                }),
            })
        }
    }

    fn parse_function_declaration(&mut self) -> Result<Decl, ParseError> {
        let name = if let Some(token) = self.advance() {
            if let TokenKind::T_IDENTIFIER(n) = &token.kind {
                n.clone()
            } else {
                let (line, col) = (token.line, token.col);
                return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
            }
        } else {
            let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
            return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
        };
        
        self.consume(&TokenKind::T_PARENL, "'(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&TokenKind::T_PARENR) {
            loop {
                let param_type = if let Some(token) = self.advance() {
                    token.kind.clone()
                } else {
                    let (line, col) = self.peek().map(|t| (t.line, t.col)).unwrap_or((0, 0));
                    return Err(ParseError { kind: ParseErrorKind::ExpectedTypeToken, line, col });
                };
                let param_name = if let Some(token) = self.advance() {
                    if let TokenKind::T_IDENTIFIER(n) = &token.kind {
                        n.clone()
                    } else {
                        let (line, col) = (token.line, token.col);
                        return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
                    }
                } else {
                    let (line, col) = self.previous().map(|t| (t.line, t.col)).unwrap_or((0, 0));
                    return Err(ParseError { kind: ParseErrorKind::ExpectedIdentifier, line, col });
                };
                
                params.push(Param {
                    name: param_name,
                    param_type,
                });
                
                if !self.match_token(&TokenKind::T_COMMA) {
                    break;
                }
            }
        }
        
        self.consume(&TokenKind::T_PARENR, "')' after parameters")?;
}