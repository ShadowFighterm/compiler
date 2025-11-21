#![allow(unused)]
use std::fmt;
use crate::lexer::LexError;
use crate::token as tk;
use tk::{Token, TokenKind};

pub struct HandLexer<'a> {
    input: &'a str,   // original input string
    chars: Vec<char>, // input as char vector for easy indexing
    pos: usize,       // currrent position in chars
    line: usize,      // error reporting: current line (1-based)
    col: usize,       // error reporting: current column (1-based)
}

impl<'a> HandLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        Self {
            input,
            chars,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn eof(&self) -> bool {
        return self.pos >= self.chars.len();
    }
    fn peek(&self) -> Option<char> {
        return self.chars.get(self.pos).copied(); // return a copy of the char or None if eof
    }
    fn peek_n(&self, n: usize) -> Option<char> {
        return self.chars.get(self.pos + n).copied(); // return a copy of the char
    }

    fn advance(&mut self) -> Option<char> {
        //advacne one character
        if self.eof() {
            return None;
        }
        let ch = self.chars[self.pos];
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        return Some(ch); // return the character we advanced over if eof return None
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexError> {
        // returns ok if success and err if fail
        loop {
            let mut progressed = false;
            // whitespace
            while matches!(self.peek(), Some(c) if c.is_whitespace()) {
                // if peek is whitespace
                progressed = true;
                self.advance();
            }
            // line comment //
            if self.peek() == Some('/') && self.peek_n(1) == Some('/') {
                progressed = true;
                self.advance();
                self.advance();
                while let Some(c) = self.peek() {
                    // while peek is not newline
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            }
            // block comment /* ... */
            else if self.peek() == Some('/') && self.peek_n(1) == Some('*') {
                progressed = true;
                let start_line = self.line;
                let start_col = self.col;
                self.advance(); // /
                self.advance(); // *
                loop {
                    if self.eof() {
                        return Err(LexError::UnterminatedComment(start_line, start_col));
                    }
                    if self.peek() == Some('*') && self.peek_n(1) == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    } else {
                        self.advance();
                    }
                }
            }
            if !progressed {
                break;
            }
        }
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while !self.eof() {
            self.skip_whitespace_and_comments()?; // ? means if error return it to the main function
            if self.eof() {
                break;
            }

            let ch = self.peek().unwrap(); // we safely can unwrap because we checked eof (unwrapping option returns the value inside or panics if None)
            let (line, col) = (self.line, self.col);

            // Strings (we emit T_QUOTES, T_STRINGLIT, T_QUOTES)
            if ch == '"' {
                tokens.push(Token::new(TokenKind::T_QUOTES, line, col));
                self.advance(); // consume opening "
                let s = self.read_string_contents()?;
                tokens.push(Token::new(TokenKind::T_STRINGLIT(s), line, col));
                // closing quote
                if self.peek() == Some('"') {
                    tokens.push(Token::new(TokenKind::T_QUOTES, self.line, self.col));
                    self.advance();
                } else {
                    return Err(LexError::UnterminatedString(line, col));
                }
                continue; // we need to check eof again after reading string
            }

            // two-char operators (longest-match)
            if let (Some(a), Some(b)) = (self.peek(), self.peek_n(1)) {
                match (a, b) {
                    ('=', '=') => {
                        tokens.push(Token::new(TokenKind::T_EQUALSOP, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('!', '=') => {
                        tokens.push(Token::new(TokenKind::T_NEQ, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('<', '=') => {
                        tokens.push(Token::new(TokenKind::T_LTE, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('>', '=') => {
                        tokens.push(Token::new(TokenKind::T_GTE, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('&', '&') => {
                        tokens.push(Token::new(TokenKind::T_ANDAND, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('|', '|') => {
                        tokens.push(Token::new(TokenKind::T_OROR, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('<', '<') => {
                        tokens.push(Token::new(TokenKind::T_LSHIFT, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    ('>', '>') => {
                        tokens.push(Token::new(TokenKind::T_RSHIFT, line, col));
                        self.advance();
                        self.advance();
                        continue;
                    }
                    _ => {}
                }
            }

            // single-char punctuation & operators
            match ch {
                '(' => {
                    tokens.push(Token::new(TokenKind::T_PARENL, line, col));
                    self.advance();
                    continue;
                }
                ')' => {
                    tokens.push(Token::new(TokenKind::T_PARENR, line, col));
                    self.advance();
                    continue;
                }
                '{' => {
                    tokens.push(Token::new(TokenKind::T_BRACEL, line, col));
                    self.advance();
                    continue;
                }
                '}' => {
                    tokens.push(Token::new(TokenKind::T_BRACER, line, col));
                    self.advance();
                    continue;
                }
                '[' => {
                    tokens.push(Token::new(TokenKind::T_BRACKETL, line, col));
                    self.advance();
                    continue;
                }
                ']' => {
                    tokens.push(Token::new(TokenKind::T_BRACKETR, line, col));
                    self.advance();
                    continue;
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::T_COMMA, line, col));
                    self.advance();
                    continue;
                }
                ';' => {
                    tokens.push(Token::new(TokenKind::T_SEMICOLON, line, col));
                    self.advance();
                    continue;
                }
                ':' => {
                    tokens.push(Token::new(TokenKind::T_COLON, line, col));
                    self.advance();
                    continue;
                }
                '.' => {
                    tokens.push(Token::new(TokenKind::T_DOT, line, col));
                    self.advance();
                    continue;
                }
                '=' => {
                    tokens.push(Token::new(TokenKind::T_ASSIGNOP, line, col));
                    self.advance();
                    continue;
                }
                '+' => {
                    tokens.push(Token::new(TokenKind::T_PLUS, line, col));
                    self.advance();
                    continue;
                }
                '-' => {
                    tokens.push(Token::new(TokenKind::T_MINUS, line, col));
                    self.advance();
                    continue;
                }
                '*' => {
                    tokens.push(Token::new(TokenKind::T_STAR, line, col));
                    self.advance();
                    continue;
                }
                '/' => {
                    tokens.push(Token::new(TokenKind::T_SLASH, line, col));
                    self.advance();
                    continue;
                }
                '%' => {
                    tokens.push(Token::new(TokenKind::T_PERCENT, line, col));
                    self.advance();
                    continue;
                }
                '^' => {
                    tokens.push(Token::new(TokenKind::T_CARET, line, col));
                    self.advance();
                    continue;
                }
                '&' => {
                    tokens.push(Token::new(TokenKind::T_AMP, line, col));
                    self.advance();
                    continue;
                }
                '|' => {
                    tokens.push(Token::new(TokenKind::T_PIPE, line, col));
                    self.advance();
                    continue;
                }
                '~' => {
                    tokens.push(Token::new(TokenKind::T_TILDE, line, col));
                    self.advance();
                    continue;
                }
                '!' => {
                    tokens.push(Token::new(TokenKind::T_NOT, line, col));
                    self.advance();
                    continue;
                }
                '<' => {
                    tokens.push(Token::new(TokenKind::T_LT, line, col));
                    self.advance();
                    continue;
                }
                '>' => {
                    tokens.push(Token::new(TokenKind::T_GT, line, col));
                    self.advance();
                    continue;
                }
                _ => {}
            }

            // numbers (digit start) first read the number if it starts with a digit
            if ch.is_ascii_digit() {
                let tok = self.read_number()?;
                tokens.push(tok);
                continue;
            }

            // identifier or keyword (unicode friendly)
            if is_identifier_start(ch) {
                let tok = self.read_identifier_or_keyword();
                tokens.push(tok);
                continue;
            }

            return Err(LexError::UnexpectedChar(ch, line, col));
        }

        tokens.push(Token::new(TokenKind::T_EOF, self.line, self.col));
        Ok(tokens)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.col;
        let mut s = String::new();
        s.push(self.advance().unwrap()); // first char guaranteed valid by caller
        while let Some(ch) = self.peek() {
            if is_identifier_part(ch) {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // keywords mapping
        let kind = match s.as_str() {
            "fn" => TokenKind::T_FUNCTION,
            "int" => TokenKind::T_INT,
            "float" => TokenKind::T_FLOAT,
            "bool" => TokenKind::T_BOOL,
            "string" => TokenKind::T_STRING,
            "return" => TokenKind::T_RETURN,
            "break" => TokenKind::T_BREAK,
            "if" => TokenKind::T_IF,
            "else" => TokenKind::T_ELSE,
            "for" => TokenKind::T_FOR,
            "while" => TokenKind::T_WHILE,
            "true" => TokenKind::T_BOOLLIT(true),
            "false" => TokenKind::T_BOOLLIT(false),
            _ => TokenKind::T_IDENTIFIER(s),
        };

        Token::new(kind, start_line, start_col)
    }

    fn read_number(&mut self) -> Result<Token, LexError> {
        let start_line = self.line;
        let start_col = self.col;
        let mut s = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // float?
        if self.peek() == Some('.') && self.peek_n(1).map(|c| c.is_ascii_digit()).unwrap_or(false) {
            s.push(self.advance().unwrap()); // consume '.'
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    s.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            // next char shouldn't be ident-start
            if let Some(nxt) = self.peek() {
                if nxt == '_' || nxt.is_alphabetic() {
                    return Err(LexError::InvalidNumber(start_line, start_col));
                }
            }
            let v: f64 = s
                .parse()
                .map_err(|_| LexError::InvalidNumber(start_line, start_col))?;
            Ok(Token::new(TokenKind::T_FLOATLIT(v), start_line, start_col))
        } else {
            if let Some(nxt) = self.peek() {
                if nxt == '_' || nxt.is_alphabetic() {
                    return Err(LexError::InvalidNumber(start_line, start_col));
                }
            }
            let v: i64 = s
                .parse()
                .map_err(|_| LexError::InvalidNumber(start_line, start_col))?;
            Ok(Token::new(TokenKind::T_INTLIT(v), start_line, start_col))
        }
    }

    fn read_string_contents(&mut self) -> Result<String, LexError> {
        // collects characters until closing quote; caller consumes quotes
        let mut buf = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                return Ok(buf);
            }
            if ch == '\\' {
                self.advance(); // consume '\'
                let esc = self
                    .peek()
                    .ok_or(LexError::UnterminatedString(self.line, self.col))?;
                match esc {
                    'n' => {
                        self.advance();
                        buf.push('\n');
                    }
                    't' => {
                        self.advance();
                        buf.push('\t');
                    }
                    'r' => {
                        self.advance();
                        buf.push('\r');
                    }
                    '\\' => {
                        self.advance();
                        buf.push('\\');
                    }
                    '"' => {
                        self.advance();
                        buf.push('"');
                    }
                    'x' => {
                        // \xHH
                        self.advance(); // x
                        let h1 = self.advance().ok_or(LexError::InvalidEscape(
                            "\\x (truncated)".into(),
                            self.line,
                            self.col,
                        ))?;
                        let h2 = self.advance().ok_or(LexError::InvalidEscape(
                            "\\x (truncated)".into(),
                            self.line,
                            self.col,
                        ))?;
                        let hs = format!("{}{}", h1, h2);
                        let byte = u8::from_str_radix(&hs, 16).map_err(|_| {
                            LexError::InvalidEscape(format!("\\x{}", hs), self.line, self.col)
                        })?;
                        buf.push(byte as char);
                    }
                    'u' => {
                        // \uXXXX (4 hex)
                        self.advance(); // u
                        let mut hexs = String::new();
                        for _ in 0..4 {
                            let h = self.advance().ok_or(LexError::InvalidEscape(
                                "\\u (truncated)".into(),
                                self.line,
                                self.col,
                            ))?;
                            hexs.push(h);
                        }
                        let cp = u32::from_str_radix(&hexs, 16).map_err(|_| {
                            LexError::InvalidEscape(format!("\\u{}", hexs), self.line, self.col)
                        })?;
                        if let Some(c) = std::char::from_u32(cp) {
                            buf.push(c);
                        } else {
                            return Err(LexError::InvalidEscape(
                                format!("\\u{}", hexs),
                                self.line,
                                self.col,
                            ));
                        }
                    }
                    other => {
                        // unknown escape: push literally (or you can return Err)
                        self.advance();
                        buf.push(other);
                    }
                }
            } else {
                buf.push(ch);
                self.advance();
            }
        }
        Err(LexError::UnterminatedString(self.line, self.col))
    }
}

// Identifier rules (Unicode-friendly)
// Start: underscore OR any Unicode letter OR any non-ASCII (bonus) but NOT a digit
fn is_identifier_start(c: char) -> bool {
    if c == '_' {
        return true;
    }
    if c.is_ascii_digit() {
        return false;
    }
    if c.is_alphabetic() {
        return true;
    }
    (c as u32) >= 0x80 // allow non-ascii letters (emoji/CJK) as bonus
}

// Part: letters, digits, underscore, or other non-ascii
fn is_identifier_part(c: char) -> bool {
    if c == '_' {
        return true;
    }
    if c.is_alphanumeric() {
        return true;
    }
    (c as u32) >= 0x80
}
