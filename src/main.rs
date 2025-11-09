#![allow(unused)]
#![allow(non_snake_case)]
mod lexer;
mod token;
mod parser;  // Add parser module

use crate::token::{Token, TokenKind};
use crate::lexer::{HandLexer, RegexLexer}; 
use crate::parser::parser::Parser;  // Import parser type
use crate::parser::ast::{Expr, Stmt, Decl, Param, Program};  // Import AST types

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("sample.src");
    let src = fs::read_to_string(path).expect("failed to read source file");

    println!("LEXING");
    let mut HandLexer = HandLexer::new(&src);
    let tokens = match HandLexer.tokenize() {
        Ok(tokens) => {
            let reprs: Vec<String> = tokens.iter().map(|t| format!("{}", t)).collect();
            println!("Tokens: [{}]", reprs.join(", "));
            tokens
        }
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            return;
        }
    };

    println!("\n PARSING ");
    let mut parser = Parser::new(&tokens);
    match parser.parse_program() {
        Ok(program) => {
            println!("Parsing successful!");
            println!("{:#?}", program);

            println!("\n=== PROGRAM STRUCTURE ===");
            print_program(&program);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}

// Helper function to pretty print the program
fn print_program(program: &Program) {
    println!("Program with {} declarations:", program.declarations.len());

    for (i, decl) in program.declarations.iter().enumerate() {
        println!("\nDeclaration {}:", i + 1);
        match decl {
            Decl::Function { name, params, return_type, body } => {
                print!("  Function: {}(", name);
                for (j, param) in params.iter().enumerate() {
                    if j > 0 { print!(", ") }
                    print!("{}: {:?}", param.name, param.param_type);
                }
                println!(")");
                if let Some(rt) = return_type {
                    println!("    Return type: {:?}", rt);
                }
                println!("    Body: {:#?}", body);
            }
            Decl::GlobalVar { name, type_annot, value } => {
                print!("  Global variable: {}", name);
                if let Some(ty) = type_annot {
                    print!(": {:?}", ty);
                }
                if let Some(val) = value {
                    println!(" = {:#?}", val);
                } else {
                    println!(" (uninitialized)");
                }
            }
        }
    }
}
