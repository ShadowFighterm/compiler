use crate::token::TokenKind;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedEOF,
    ExpectedIdentifier,
    ExpectedTypeToken,
    ExpectedExpr,
    UnexpectedToken(TokenKind),
    Expected(String), 
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseErrorKind::ExpectedIdentifier => write!(f, "Expected identifier"),
            ParseErrorKind::ExpectedTypeToken => write!(f, "Expected type token"),
            ParseErrorKind::ExpectedExpr => write!(f, "Expected expression"),
            ParseErrorKind::UnexpectedToken(kind) => write!(f, "Unexpected token: {}", kind),
            ParseErrorKind::Expected(msg) => write!(f, "{}", msg),
        }?;
        write!(f, " at line {}, column {}", self.line, self.col)
    }
}
