#![allow(unused)]
#![allow(non_snake_case)]
mod lexer;
mod token;
mod ir;
mod parser;  // Add parser module
mod semantics;
use crate::semantics::scope;
use crate::semantics::typechecker::{TypeChkError};  // Import TypeChecker and TypeChkError
use crate::token::{Token, TokenKind};
use crate::lexer::{HandLexer, RegexLexer};
use crate::parser::parser::Parser;  // Import parser type
use crate::parser::ast::{Expr, Stmt, Decl, Param, Program};  // Import AST types
use crate::semantics::scope::scope::{ScopeStack, Symbol, SymbolKind, Type, ScopeError};
use crate::ir::ir_generator::IRGenerator;

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

            println!("\n=== SEMANTIC ANALYSIS ===");
            let (scope_errors, typechk_errors) = perform_semantic_analysis(&program);

            // Print semantic errors here...

            println!("\n=== INTERMEDIATE REPRESENTATION (TAC) ===");
            let mut irgen = IRGenerator::new();
            for decl in &program.declarations {
                match decl {
                    Decl::Stmt(stmt) => irgen.generate_statement(stmt),
                    Decl::Function { body, .. } => irgen.generate_statement(body),
                    Decl::GlobalVar { value, name, .. } => {
                        if let Some(expr) = value {
                            let pseudo_let = Stmt::Let {
                                name: name.clone(),
                                type_annot: None,
                                value: expr.clone(),
                            };
                            irgen.generate_statement(&pseudo_let);
                        }
                    }
                }
            }
            irgen.print_ir_code();
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
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
            Decl::Stmt(stmt) => {
                println!("  Global statement: {:#?}", stmt);
            }
        }
    }
}

// Perform combined scope and type checking, collect errors from both
fn perform_semantic_analysis(program: &Program) -> (Vec<ScopeError>, Vec<TypeChkError>) {
    let mut scope_stack = ScopeStack::new();
    let mut scope_errors = Vec::new();
    let mut typechk_errors = Vec::new();

    scope_stack.enter_scope();

    for decl in &program.declarations {
        match decl {
            Decl::GlobalVar { name, type_annot, value } => {
                let ty = if let Some(t) = type_annot { token_to_type(t.clone()) } else { Type::Void };
                let initialized = value.is_some();

                if scope_stack.insert_variable(name.clone(), ty.clone(), true, initialized).is_err() {
                    scope_errors.push(ScopeError::VariableRedefinition);
                }
                if let Some(val) = value {
                    analyze_expr(val, &mut scope_stack, &mut scope_errors, &mut typechk_errors);
                }
            }
            Decl::Function { name, params, return_type, body } => {
                let param_types: Vec<Type> = params.iter().map(|p| token_to_type(p.param_type.clone())).collect();
                let ret_ty = if let Some(t) = return_type { token_to_type(t.clone()) } else { Type::Void };
                if scope_stack.insert_function_definition(name.clone(), param_types.clone(), ret_ty.clone()).is_err() {
                    scope_errors.push(ScopeError::FunctionRedefinition);
                } else {
                    scope_stack.enter_scope();
                    for (p, ty) in params.iter().zip(param_types.iter()) {
                        let _ = scope_stack.insert_variable(p.name.clone(), ty.clone(), false, true);
                    }
                    analyze_stmt(body, &mut scope_stack, &mut scope_errors, &mut typechk_errors, Some(&ret_ty));
                    scope_stack.exit_scope();
                }
            }
            Decl::Stmt(stmt) => {
                analyze_stmt(stmt, &mut scope_stack, &mut scope_errors, &mut typechk_errors, None);
            }
        }
    }

    scope_stack.exit_scope();

    (scope_errors, typechk_errors)
}

fn analyze_stmt(stmt: &Stmt,
                scope_stack: &mut ScopeStack,
                scope_errors: &mut Vec<ScopeError>,
                typechk_errors: &mut Vec<TypeChkError>,
                current_return_type: Option<&Type>) {
    match stmt {
        Stmt::Expr(expr) => {
            analyze_expr(expr, scope_stack, scope_errors, typechk_errors);
        }
        Stmt::Let { name, type_annot, value } => {
            analyze_expr(value, scope_stack, scope_errors, typechk_errors);
            let ty = type_annot.as_ref().map(|t| token_to_type(t.clone())).unwrap_or(Type::Void);
            if let Err(e) = scope_stack.insert_variable(name.clone(), ty, false, true) {
                scope_errors.push(e);
            }
        }
        Stmt::Block(stmts) => {
            scope_stack.enter_scope();
            for s in stmts {
                analyze_stmt(s, scope_stack, scope_errors, typechk_errors, current_return_type);
            }
            scope_stack.exit_scope();
        }
        Stmt::Return(expr_opt) => {
            if let Some(expr) = expr_opt {
                let ty = match visit_expr(expr, scope_stack) {
                    Ok(t) => t,
                    Err(e) => {
                        typechk_errors.push(e);
                        Type::Void
                    }
                };
                if let Some(expected) = current_return_type {
                    if *expected != Type::Unknown && *expected != Type::Void && *expected != ty {
                        typechk_errors.push(TypeChkError::ErroneousReturnType);
                    }
                }
            } else {
                if let Some(expected) = current_return_type {
                    if *expected != Type::Void && *expected != Type::Unknown {
                        typechk_errors.push(TypeChkError::ErroneousReturnType);
                    }
                }
            }
        }
        Stmt::Break => {
            if !scope_stack.in_loop() {
                typechk_errors.push(TypeChkError::ErroneousBreak);
            }
        }
        Stmt::If { condition, then_branch, else_branch } => {
            match visit_expr(condition, scope_stack) {
                Ok(t) => if t != Type::Bool {
                    typechk_errors.push(TypeChkError::NonBooleanCondStmt);
                },
                Err(e) => typechk_errors.push(e),
            }
            analyze_stmt(then_branch, scope_stack, scope_errors, typechk_errors, current_return_type);
            if let Some(else_stmt) = else_branch {
                analyze_stmt(else_stmt, scope_stack, scope_errors, typechk_errors, current_return_type);
            }
        }
        Stmt::While { condition, body } => {
            match visit_expr(condition, scope_stack) {
                Ok(t) => if t != Type::Bool {
                    typechk_errors.push(TypeChkError::NonBooleanCondStmt);
                },
                Err(e) => typechk_errors.push(e),
            }
            scope_stack.enter_loop();
            analyze_stmt(body, scope_stack, scope_errors, typechk_errors, current_return_type);
            scope_stack.exit_loop();
        }
        Stmt::For { init, condition, increment, body } => {
            scope_stack.enter_scope();
            scope_stack.enter_loop();

            if let Some(init_stmt) = init {
                analyze_stmt(init_stmt, scope_stack, scope_errors, typechk_errors, current_return_type);
            }
            if let Some(cond) = condition {
                match visit_expr(cond, scope_stack) {
                    Ok(t) => if t != Type::Bool {
                        typechk_errors.push(TypeChkError::NonBooleanCondStmt);
                    },
                    Err(e) => typechk_errors.push(e),
                }
            }
            if let Some(incr) = increment {
                analyze_expr(incr, scope_stack, scope_errors, typechk_errors);
            }
            analyze_stmt(body, scope_stack, scope_errors, typechk_errors, current_return_type);

            scope_stack.exit_scope();
            scope_stack.exit_loop();
        }
        Stmt::Function { name, params, return_type, body } => {
            // Nested functions ignored here as they are handled via declaration processing
        }
    }
}

fn analyze_expr(expr: &Expr, scope_stack: &mut ScopeStack, scope_errors: &mut Vec<ScopeError>, typechk_errors: &mut Vec<TypeChkError>) {
    // Scope error checks
    match expr {
        Expr::Identifier(name) => {
            if let Err(e) = scope_stack.lookup_variable(name) {
                scope_errors.push(e);
            }
        }
        Expr::Binary { left, right, .. } => {
            analyze_expr(left, scope_stack, scope_errors, typechk_errors);
            analyze_expr(right, scope_stack, scope_errors, typechk_errors);
        }
        Expr::Unary { expr: inner, .. } => {
            analyze_expr(inner, scope_stack, scope_errors, typechk_errors);
        }
        Expr::Call { callee, args } => {
            analyze_expr(callee, scope_stack, scope_errors, typechk_errors);
            for arg in args {
                analyze_expr(arg, scope_stack, scope_errors, typechk_errors);
            }
            if let Expr::Identifier(name) = &**callee {
                if let Err(e) = scope_stack.lookup_function(name) {
                    scope_errors.push(e);
                }
            }
        }
        Expr::Grouping(inner) => {
            analyze_expr(inner, scope_stack, scope_errors, typechk_errors);
        }
        Expr::Integer(_) | Expr::Float(_) | Expr::Boolean(_) | Expr::StringLit(_) => {}
    }

    // Type checking errors
    match visit_expr(expr, scope_stack) {
        Ok(_) => {}
        Err(e) => typechk_errors.push(e)
    }
}

// Core type checker logic for expressions - returns type or TypeChkError
fn visit_expr(expr: &Expr, scope_stack: &mut ScopeStack) -> Result<Type, TypeChkError> {
    match expr {
        Expr::Identifier(name) => {
            match scope_stack.lookup_variable(name) {
                Ok(sym) => {
                    if let Some(ty) = &sym.ty {
                        Ok(ty.clone())
                    } else {
                        Ok(Type::Unknown)
                    }
                }
                Err(_) => Err(TypeChkError::ErroneousVarDecl)
            }
        }
        Expr::Integer(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::Boolean(_) => Ok(Type::Bool),
        Expr::StringLit(_) => Ok(Type::String),

        Expr::Unary { operator, expr: inner } => {
            let t = visit_expr(inner, scope_stack)?;
            match operator {
                TokenKind::T_MINUS => {
                    if matches!(t, Type::Int | Type::Float) {
                        Ok(t)
                    } else {
                        Err(TypeChkError::AttemptedAddOpOnNonNumeric)
                    }
                }
                TokenKind::T_NOT => {
                    if t == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeChkError::AttemptedBoolOpOnNonBools)
                    }
                }
                _ => Ok(t),
            }
        }

        Expr::Binary { left, operator, right } => {
            let lt = visit_expr(left, scope_stack)?;
            let rt = visit_expr(right, scope_stack)?;

            match operator {
                TokenKind::T_PLUS | TokenKind::T_MINUS | TokenKind::T_STAR | TokenKind::T_SLASH | TokenKind::T_PERCENT => {
                    if !matches!(lt, Type::Int | Type::Float) || !matches!(rt, Type::Int | Type::Float) {
                        return Err(TypeChkError::AttemptedAddOpOnNonNumeric);
                    }
                    if lt == Type::Float || rt == Type::Float { Ok(Type::Float) } else { Ok(Type::Int) }
                }
                TokenKind::T_CARET => {
                    if !matches!(lt, Type::Int | Type::Float) || !matches!(rt, Type::Int | Type::Float) {
                        return Err(TypeChkError::AttemptedExponentiationOfNonNumeric);
                    }
                    if lt == Type::Float || rt == Type::Float { Ok(Type::Float) } else { Ok(Type::Int) }
                }
                TokenKind::T_ANDAND | TokenKind::T_OROR => {
                    if lt != Type::Bool || rt != Type::Bool {
                        return Err(TypeChkError::AttemptedBoolOpOnNonBools);
                    }
                    Ok(Type::Bool)
                }
                TokenKind::T_LSHIFT | TokenKind::T_RSHIFT => {
                    if lt != Type::Int || rt != Type::Int {
                        return Err(TypeChkError::AttemptedShiftOnNonInt);
                    }
                    Ok(Type::Int)
                }
                TokenKind::T_EQUALSOP | TokenKind::T_NEQ | TokenKind::T_LT | TokenKind::T_GT | TokenKind::T_LTE | TokenKind::T_GTE => {
                    if lt == Type::Unknown || rt == Type::Unknown {
                        if lt != rt && lt != Type::Unknown && rt != Type::Unknown {
                            return Err(TypeChkError::ExpressionTypeMismatch);
                        } else {
                            Ok(Type::Bool)
                        }
                    } else if lt != rt {
                        return Err(TypeChkError::ExpressionTypeMismatch);
                    } else {
                        Ok(Type::Bool)
                    }
                }
                _ => Ok(Type::Unknown),
            }
        }

            Expr::Call { callee, args } => {
                if let Expr::Identifier(name) = &**callee {
                    let func_sym = match scope_stack.lookup_function(name) {
                        Ok(sym) => sym,
                        Err(_) => return Err(TypeChkError::ErroneousVarDecl),
                    };
                    let (param_types, ret_type) = if let SymbolKind::Function { params, return_type, .. } = &func_sym.kind {
                        (params.clone(), return_type.clone())
                    } else {
                        return Err(TypeChkError::ErroneousVarDecl);
                    };
                    drop(func_sym);

                    if param_types.len() != args.len() {
                        return Err(TypeChkError::FnCallParamCount);
                    }

                    for (i, arg) in args.iter().enumerate() {
                        let arg_ty = visit_expr(arg, scope_stack)?;
                        let param_ty = param_types.get(i).cloned().unwrap_or(Type::Unknown);
                        if param_ty != Type::Unknown && arg_ty != Type::Unknown && param_ty != arg_ty {
                            return Err(TypeChkError::FnCallParamType);
                        }
                    }
                    return Ok(ret_type);
                }
                let _ = visit_expr(callee, scope_stack)?;
                for a in args {
                    let _ = visit_expr(a, scope_stack)?;
                }
                Ok(Type::Unknown)
            }

        Expr::Grouping(inner) => visit_expr(inner, scope_stack),

        _ => Ok(Type::Unknown),
    }
}

fn token_to_type(token: TokenKind) -> Type {
    match token {
        TokenKind::T_INT => Type::Int,
        TokenKind::T_FLOAT => Type::Float,
        TokenKind::T_BOOL => Type::Bool,
        TokenKind::T_STRING => Type::Custom("String".to_string()), 
        _ => Type::Void, // Default fallback type
    }
}
