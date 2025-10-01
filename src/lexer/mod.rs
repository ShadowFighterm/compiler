mod hand;
mod regex;
mod error;

pub use hand::HandLexer as HandLexer;
pub use regex::RegexLexer as RegexLexer;
pub use error::LexError as LexError;

// use crate::token::{Token};

// pub trait LexerTrait {
//     fn tokenize(&mut self) -> Result<Vec<Token>, LexError>;
// }
