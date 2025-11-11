#![allow(unused)]
#![allow(non_snake_case)]
mod lexer;
mod token;
mod parser;  // Add parser module
mod semantics;
use crate::token::{Token, TokenKind};
use crate::lexer::{HandLexer, RegexLexer};
use crate::parser::parser::Parser;  // Import parser type
use crate::parser::ast::{Expr, Stmt, Decl, Param, Program};  // Import AST types
use semantics::scope::scope::{ScopeStack, Symbol, SymbolKind, Type, ScopeError};
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

            println!("\n=== SCOPE ANALYSIS ===");
            let errors = perform_scope_analysis(&program);
            if errors.is_empty() {
                println!("Scope analysis successful! No errors found.");
            } else {
                println!("Scope analysis found {} errors:", errors.len());
                for error in &errors {
                    println!("  {:?}", error);
                }
            }
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

fn perform_scope_analysis(program: &Program) -> Vec<ScopeError> {
    let mut scope_stack = ScopeStack::new();
    let mut errors = Vec::new();

    // Enter global scope
    scope_stack.enter_scope();

    // Process global declarations
    for decl in &program.declarations {
        match decl {
            Decl::GlobalVar { name, type_annot, value } => {
                let ty = type_annot.as_ref().map(|t| token_to_type(t.clone())).unwrap_or(Type::Void);
                let initialized = value.is_some();
                if let Err(e) = scope_stack.insert_variable(name.clone(), ty, true, initialized) {
                    errors.push(e);
                }
                if let Some(val) = value {
                    analyze_expr(val, &mut scope_stack, &mut errors);
                }
            }
            Decl::Function { name, params, return_type, body } => {
                let param_types: Vec<Type> = params.iter().map(|p| token_to_type(p.param_type.clone())).collect();
                let ret_ty = return_type.as_ref().map(|t| token_to_type(t.clone())).unwrap_or(Type::Void);
                if let Err(e) = scope_stack.insert_function_definition(name.clone(), param_types, ret_ty) {
                    errors.push(e);
                } else {
                    // Analyze function body
                    scope_stack.enter_scope();
                    // Insert parameters
                    for param in params {
                        let ty = token_to_type(param.param_type.clone());
                        if let Err(e) = scope_stack.insert_variable(param.name.clone(), ty, false, true) {
                            errors.push(e);
                        }
                    }
                    // Analyze body statements
                    analyze_stmt(body, &mut scope_stack, &mut errors);
                    scope_stack.exit_scope();
                }
            }
        }
    }

    scope_stack.exit_scope(); // Exit global scope
    errors
}

fn analyze_stmt(stmt: &Stmt, scope_stack: &mut ScopeStack, errors: &mut Vec<ScopeError>) {
    match stmt {
        Stmt::Expr(expr) => {
            analyze_expr(expr, scope_stack, errors);
        }
        Stmt::Let { name, type_annot, value } => {
            analyze_expr(value, scope_stack, errors);
            let ty = type_annot.as_ref().map(|t| token_to_type(t.clone())).unwrap_or(Type::Void);
            if let Err(e) = scope_stack.insert_variable(name.clone(), ty, true, true) {
                errors.push(e);
            }
        }
        Stmt::Block(stmts) => {
            scope_stack.enter_scope();
            for s in stmts {
                analyze_stmt(s, scope_stack, errors);
            }
            scope_stack.exit_scope();
        }
        Stmt::Return(expr_opt) => {
            if let Some(expr) = expr_opt {
                analyze_expr(expr, scope_stack, errors);
            }
        }
        Stmt::If { condition, then_branch, else_branch } => {
            analyze_expr(condition, scope_stack, errors);
            analyze_stmt(then_branch, scope_stack, errors);
            if let Some(else_stmt) = else_branch {
                analyze_stmt(else_stmt, scope_stack, errors);
            }
        }
        Stmt::While { condition, body } => {
            analyze_expr(condition, scope_stack, errors);
            analyze_stmt(body, scope_stack, errors);
        }
        Stmt::For { init, condition, increment, body } => {
            scope_stack.enter_scope();
            if let Some(init_stmt) = init {
                analyze_stmt(init_stmt, scope_stack, errors);
            }
            if let Some(cond) = condition {
                analyze_expr(cond, scope_stack, errors);
            }
            if let Some(incr) = increment {
                analyze_expr(incr, scope_stack, errors);
            }
            analyze_stmt(body, scope_stack, errors);
            scope_stack.exit_scope();
        }
        Stmt::Function { name, params, return_type, body } => {
            // Nested function? Assuming not, but handle as prototype for now
            let param_types: Vec<Type> = params.iter().map(|p| token_to_type(p.param_type.clone())).collect();
            let ret_ty = return_type.as_ref().map(|t| token_to_type(t.clone())).unwrap_or(Type::Void);
            if let Err(e) = scope_stack.insert_function_prototype(name.clone(), param_types, ret_ty) {
                errors.push(e);
            }
            // Analyze body if needed, but for simplicity, skip nested functions
        }
    }
}

fn analyze_expr(expr: &Expr, scope_stack: &mut ScopeStack, errors: &mut Vec<ScopeError>) {
    match expr {
        Expr::Identifier(name) => {
            if let Err(e) = scope_stack.lookup_variable(name) {
                errors.push(e);
            }
        }
        Expr::Binary { left, right, .. } => {
            analyze_expr(left, scope_stack, errors);
            analyze_expr(right, scope_stack, errors);
        }
        Expr::Unary { expr: inner, .. } => {
            analyze_expr(inner, scope_stack, errors);
        }
        Expr::Call { callee, args } => {
            analyze_expr(callee, scope_stack, errors);
            for arg in args {
                analyze_expr(arg, scope_stack, errors);
            }
            // For function calls, check if callee is a function, but since no type checking, just ensure it's declared
            if let Expr::Identifier(name) = callee.as_ref() {
                if let Err(e) = scope_stack.lookup_function(name) {
                    errors.push(e);
                }
            }
        }
        Expr::Grouping(inner) => {
            analyze_expr(inner, scope_stack, errors);
        }
        // Literals: no scope issues
        Expr::Integer(_) | Expr::Float(_) | Expr::Boolean(_) | Expr::StringLit(_) => {}
    }
}

fn token_to_type(token: TokenKind) -> Type {
    match token {
        TokenKind::T_INT => Type::Int,
        TokenKind::T_FLOAT => Type::Float,
        TokenKind::T_BOOL => Type::Bool,
        TokenKind::T_STRING => Type::Custom("String".to_string()), // Assuming string is custom
        _ => Type::Void, // Default
    }
}

