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

}