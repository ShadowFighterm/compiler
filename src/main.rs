#![allow(unused)]
mod lexer;
mod token;
mod ast;
mod parser;

use crate::token::{Token, TokenKind};
use crate::parser::Parser;
use lexer::{HandLexer, RegexLexer}; // for later
use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let which = args.next().unwrap_or_else(|| "hand".to_string());
    let path = args.next().expect("Usage: <hand|regex> <file>");
    let src = fs::read_to_string(&path).expect("read source file");

    if which == "hand" {
        let mut lx = HandLexer::new(&src);
        match lx.tokenize() {
            Ok(tokens) => {
                let out = tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Tokens: [{}]", out);

                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => println!("AST: {:#?}", ast),
                    Err(e) => eprintln!("Parse error: {:?}", e),
                }
            }
            Err(e) => eprintln!("Lex error: {}", e),
        }
    } else if which == "regex" {
        let mut lx = RegexLexer::new();
        match lx.tokenize(&src) {
            Ok(tokens) => {
                let out = tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Tokens: [{}]", out);

                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => println!("AST: {:#?}", ast),
                    Err(e) => eprintln!("Parse error: {:?}", e),
                }
            }
            Err(e) => eprintln!("Lex error: {}", e),
        }
    } else {
        eprintln!("Unknown lexer '{}'", which);
    }
}








// #![allow(unused)]
// mod lexer;
// mod token;
// use crate::token::{Token, TokenKind};
// use lexer::{HandLexer, RegexLexer}; // if you have RegexLexer later
// use std::env;
// use std::fs;

// fn main() {
//     let mut args = env::args().skip(1);
//     let which = args.next().unwrap_or_else(|| "hand".to_string());
//     let path = args.next().expect("Usage: <hand|regex> <file>");
//     let src = fs::read_to_string(&path).expect("read source file");

//     if which == "hand" {
//         let mut lx = HandLexer::new(&src);
//         match lx.tokenize() {
//             Ok(tokens) => {
//                 let out = tokens
//                     .iter()
//                     .map(|t| t.to_string())
//                     .collect::<Vec<_>>()
//                     .join(", ");
//                 println!("[{}]", out);
//             }
//             Err(e) => eprintln!("Lex error: {}", e),
//         }
//     } else if which == "regex" {
//         let mut lx = RegexLexer::new();
//         match lx.tokenize(&src) {
//             Ok(tokens) => {
//                 let out = tokens
//                     .iter()
//                     .map(|t| t.to_string())
//                     .collect::<Vec<_>>()
//                     .join(", ");
//                 println!("[{}]", out);
//             }
//             Err(e) => eprintln!("Lex error: {}", e),
//         }
//     } else {
//         eprintln!("Unknown lexer '{}'", which);
//     }
// }
