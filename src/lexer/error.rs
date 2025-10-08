#![allow(unused)]
use std::fmt;
#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(char, usize, usize),
    UnterminatedString(usize, usize),
    UnterminatedComment(usize, usize),
    InvalidEscape(String, usize, usize),
    InvalidNumber(usize, usize),
    InvalidIdentifierStart(char, usize, usize),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedChar(c, l, ccol) => {
                write!(f, "Unexpected character '{}' at {}:{}", c, l, ccol)
            }
            LexError::UnterminatedString(l, ccol) => {
                write!(f, "Unterminated string starting at {}:{}", l, ccol)
            }
            LexError::UnterminatedComment(l, ccol) => {
                write!(f, "Unterminated comment starting at {}:{}", l, ccol)
            }
            LexError::InvalidEscape(s, l, ccol) => {
                write!(f, "Invalid escape {} at {}:{}", s, l, ccol)
            }
            LexError::InvalidNumber(l, ccol) => {
                write!(f, "Invalid number starting at {}:{}", l, ccol)
            }
            LexError::InvalidIdentifierStart(ch, l, ccol) => {
                write!(f, "Invalid identifier start '{}' at {}:{}", ch, l, ccol)
            }
        }
    }
}
