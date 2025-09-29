use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords / types
    T_FUNCTION,
    T_INT,
    T_FLOAT,
    T_BOOL,
    T_STRING,
    T_RETURN,
    T_IF,
    T_ELSE,
    T_FOR,
    T_WHILE,
    T_BREAK,


    // Identifiers & literals
    T_IDENTIFIER(String),
    T_INTLIT(i64),
    T_FLOATLIT(f64),
    T_STRINGLIT(String),
    T_BOOLLIT(bool),

    // Punctuation
    T_PARENL,
    T_PARENR,
    T_BRACEL,
    T_BRACER,
    T_BRACKETL,
    T_BRACKETR,
    T_COMMA,
    T_SEMICOLON,
    T_COLON,
    T_DOT,
    T_QUOTES, // to match the example (quote tokens around strings)

    // Operators
    T_ASSIGNOP,
    T_EQUALSOP,
    T_NEQ,
    T_LT,
    T_GT,
    T_LTE,
    T_GTE,
    T_ANDAND,
    T_OROR,
    T_LSHIFT,
    T_RSHIFT,
    T_PLUS,
    T_MINUS,
    T_STAR,
    T_SLASH,
    T_PERCENT,
    T_CARET,
    T_AMP,
    T_PIPE,
    T_TILDE,
    T_NOT,


    T_EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    // by default struct fields are private
    pub kind: TokenKind,
    pub line: usize, // row number (1-based)
    pub col: usize,  // column number (1-based)
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Result either Ok(()) or Err
        use TokenKind::*; // bring all variants into scope no need for TokenKind::
        match self {
            // like switch on self but safe
            T_FUNCTION => write!(f, "T_FUNCTION"), // if self means TokenKind is T_FUNCTION
            T_INT => write!(f, "T_INT"),
            T_FLOAT => write!(f, "T_FLOAT"),
            T_BOOL => write!(f, "T_BOOL"),
            T_STRING => write!(f, "T_STRING"),
            T_RETURN => write!(f, "T_RETURN"),
            T_IF => write!(f, "T_IF"),
            T_ELSE => write!(f, "T_ELSE"),
            T_FOR => write!(f, "T_FOR"),
            T_WHILE => write!(f, "T_WHILE"),
            T_BREAK => write!(f, "T_BREAK"),


            T_IDENTIFIER(s) => write!(f, "T_IDENTIFIER(\"{}\")", s),
            T_INTLIT(v) => write!(f, "T_INTLIT({})", v),
            T_FLOATLIT(v) => write!(f, "T_FLOATLIT({})", v),
            T_STRINGLIT(s) => write!(f, "T_STRINGLIT(\"{}\")", s),
            T_BOOLLIT(b) => write!(f, "T_BOOLLIT({})", b),

            T_PARENL => write!(f, "T_PARENL"),
            T_PARENR => write!(f, "T_PARENR"),
            T_BRACEL => write!(f, "T_BRACEL"),
            T_BRACER => write!(f, "T_BRACER"),
            T_BRACKETL => write!(f, "T_BRACKETL"),
            T_BRACKETR => write!(f, "T_BRACKETR"),
            T_COMMA => write!(f, "T_COMMA"),
            T_SEMICOLON => write!(f, "T_SEMICOLON"),
            T_COLON => write!(f, "T_COLON"),
            T_DOT => write!(f, "T_DOT"),
            T_QUOTES => write!(f, "T_QUOTES"),

            T_ASSIGNOP => write!(f, "T_ASSIGNOP"),
            T_EQUALSOP => write!(f, "T_EQUALSOP"),
            T_NEQ => write!(f, "T_NEQ"),
            T_LT => write!(f, "T_LT"),
            T_GT => write!(f, "T_GT"),
            T_LTE => write!(f, "T_LTE"),
            T_GTE => write!(f, "T_GTE"),
            T_ANDAND => write!(f, "T_ANDAND"),
            T_OROR => write!(f, "T_OROR"),
            T_LSHIFT => write!(f, "T_LSHIFT"),
            T_RSHIFT => write!(f, "T_RSHIFT"),
            T_PLUS => write!(f, "T_PLUS"),
            T_MINUS => write!(f, "T_MINUS"),
            T_STAR => write!(f, "T_STAR"),
            T_SLASH => write!(f, "T_SLASH"),
            T_PERCENT => write!(f, "T_PERCENT"),
            T_CARET => write!(f, "T_CARET"),
            T_AMP => write!(f, "T_AMP"),
            T_PIPE => write!(f, "T_PIPE"),
            T_TILDE => write!(f, "T_TILDE"),
            T_NOT => write!(f, "T_NOT"),

            T_EOF => write!(f, "T_EOF"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = &self.kind;
        write!(f, "{kind}")
    }
}