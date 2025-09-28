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

}