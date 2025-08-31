#![allow(unused)]
use regex::Regex;
use crate::token::{Token, TokenKind};
use crate::lexer::LexError;
pub struct Rule {
    regex: Regex,
    kind: fn(&str) -> TokenKind, // function to convert matched text into TokenKind
}

pub fn build_rules() -> Vec<Rule> {
    vec![
        // Keywords
        Rule { regex: Regex::new(r"^fn\b").unwrap(), kind: |_| TokenKind::T_FUNCTION },
        Rule { regex: Regex::new(r"^int\b").unwrap(), kind: |_| TokenKind::T_INT },
        Rule { regex: Regex::new(r"^float\b").unwrap(), kind: |_| TokenKind::T_FLOAT },
        Rule { regex: Regex::new(r"^bool\b").unwrap(), kind: |_| TokenKind::T_BOOL },
        Rule { regex: Regex::new(r"^string\b").unwrap(), kind: |_| TokenKind::T_STRING },
        Rule { regex: Regex::new(r"^return\b").unwrap(), kind: |_| TokenKind::T_RETURN },
        Rule { regex: Regex::new(r"^if\b").unwrap(), kind: |_| TokenKind::T_IF },
        Rule { regex: Regex::new(r"^else\b").unwrap(), kind: |_| TokenKind::T_ELSE },
        Rule { regex: Regex::new(r"^for\b").unwrap(), kind: |_| TokenKind::T_FOR },
        Rule { regex: Regex::new(r"^while\b").unwrap(), kind: |_| TokenKind::T_WHILE },

        // Identifiers
        Rule { regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
               kind: |s| TokenKind::T_IDENTIFIER(s.to_string()) },

        // Literals
        Rule { regex: Regex::new(r"^\d+\.\d+").unwrap(),
               kind: |s| TokenKind::T_FLOATLIT(s.parse().unwrap()) },
        Rule { regex: Regex::new(r"^\d+").unwrap(),
               kind: |s| TokenKind::T_INTLIT(s.parse().unwrap()) },
        Rule { regex: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
               kind: |s| TokenKind::T_STRINGLIT(s.to_string()) },

        // Operators & punctuation
        Rule { regex: Regex::new(r"^==").unwrap(), kind: |_| TokenKind::T_EQUALSOP },
        Rule { regex: Regex::new(r"^!=").unwrap(), kind: |_| TokenKind::T_NEQ },
        Rule { regex: Regex::new(r"^<=").unwrap(), kind: |_| TokenKind::T_LTE },
        Rule { regex: Regex::new(r"^>=").unwrap(), kind: |_| TokenKind::T_GTE },
        Rule { regex: Regex::new(r"^&&").unwrap(), kind: |_| TokenKind::T_ANDAND },
        Rule { regex: Regex::new(r"^\|\|").unwrap(), kind: |_| TokenKind::T_OROR },
        Rule { regex: Regex::new(r"^<<").unwrap(), kind: |_| TokenKind::T_LSHIFT },
        Rule { regex: Regex::new(r"^>>").unwrap(), kind: |_| TokenKind::T_RSHIFT },

        Rule { regex: Regex::new(r"^=").unwrap(), kind: |_| TokenKind::T_ASSIGNOP },
        Rule { regex: Regex::new(r"^<").unwrap(), kind: |_| TokenKind::T_LT },
        Rule { regex: Regex::new(r"^>").unwrap(), kind: |_| TokenKind::T_GT },
        Rule { regex: Regex::new(r"^\+").unwrap(), kind: |_| TokenKind::T_PLUS },
        Rule { regex: Regex::new(r"^-").unwrap(), kind: |_| TokenKind::T_MINUS },
        Rule { regex: Regex::new(r"^\*").unwrap(), kind: |_| TokenKind::T_STAR },
        Rule { regex: Regex::new(r"^/").unwrap(), kind: |_| TokenKind::T_SLASH },
        Rule { regex: Regex::new(r"^%").unwrap(), kind: |_| TokenKind::T_PERCENT },
        Rule { regex: Regex::new(r"^\^").unwrap(), kind: |_| TokenKind::T_CARET },
        Rule { regex: Regex::new(r"^&").unwrap(), kind: |_| TokenKind::T_AMP },
        Rule { regex: Regex::new(r"^\|").unwrap(), kind: |_| TokenKind::T_PIPE },
        Rule { regex: Regex::new(r"^~").unwrap(), kind: |_| TokenKind::T_TILDE },
        Rule { regex: Regex::new(r"^!").unwrap(), kind: |_| TokenKind::T_NOT },

        Rule { regex: Regex::new(r"^\(").unwrap(), kind: |_| TokenKind::T_PARENL },
        Rule { regex: Regex::new(r"^\)").unwrap(), kind: |_| TokenKind::T_PARENR },
        Rule { regex: Regex::new(r"^\{").unwrap(), kind: |_| TokenKind::T_BRACEL },
        Rule { regex: Regex::new(r"^\}").unwrap(), kind: |_| TokenKind::T_BRACER },
        Rule { regex: Regex::new(r"^\[").unwrap(), kind: |_| TokenKind::T_BRACKETL },
        Rule { regex: Regex::new(r"^\]").unwrap(), kind: |_| TokenKind::T_BRACKETR },
        Rule { regex: Regex::new(r"^,").unwrap(), kind: |_| TokenKind::T_COMMA },
        Rule { regex: Regex::new(r"^;").unwrap(), kind: |_| TokenKind::T_SEMICOLON },
        Rule { regex: Regex::new(r"^:").unwrap(), kind: |_| TokenKind::T_COLON },
        Rule { regex: Regex::new(r"^\.").unwrap(), kind: |_| TokenKind::T_DOT },
    ]
}

pub struct RegexLexer {
    rules: Vec<Rule>,
}

impl RegexLexer {
    pub fn new() -> Self {
        Self { rules: build_rules() }
    }

    pub fn tokenize(&self, mut input: &str) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        let mut line = 1;
        let mut col = 1;
    
        let ws_re = Regex::new(r"^\s+").unwrap();
        let slc_re = Regex::new(r"^//[^\n]*").unwrap();
        let mlc_re = Regex::new(r"^/\*([^*]|\*+[^*/])*\*+/").unwrap();
    
        while !input.is_empty() {
            // Skip whitespace
            if let Some(m) = ws_re.find(input) {
                let lexeme = &input[..m.end()];
                for ch in lexeme.chars() {
                    if ch == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                input = &input[m.end()..];
                continue;
            }
    
            // Skip single-line comments
            if let Some(m) = slc_re.find(input) {
                let lexeme = &input[..m.end()];
                for ch in lexeme.chars() {
                    if ch == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                input = &input[m.end()..];
                continue;
            }
    
            // Skip multi-line comments
            if let Some(m) = mlc_re.find(input) {
                let lexeme = &input[..m.end()];
                for ch in lexeme.chars() {
                    if ch == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                input = &input[m.end()..];
                continue;
            }
    
            // --- Try token rules ---
            let mut matched = false;
            for rule in &self.rules {
                if let Some(m) = rule.regex.find(input) {
                    let lexeme = &input[..m.end()];
                    let kind = (rule.kind)(lexeme);
    
                    tokens.push(Token {
                        kind,
                        line,
                        col,
                    });
    
                    // Update line/col
                    for ch in lexeme.chars() {
                        if ch == '\n' {
                            line += 1;
                            col = 1;
                        } else {
                            col += 1;
                        }
                    }
    
                    input = &input[m.end()..];
                    matched = true;
                    break;
                }
            }
    
            if !matched {
                return Err(LexError::UnexpectedChar(input.chars().next().unwrap(), line, col));
            }
        }
    
        tokens.push(Token { kind: TokenKind::T_EOF, line, col });
        Ok(tokens)
    }
    
    
}