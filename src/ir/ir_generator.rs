use crate::ir::quad::Quad;
use crate::parser::ast::{Expr, Stmt};

pub struct IRGenerator {
    pub quads: Vec<Quad>,
    temp_counter: usize,
    label_counter: usize,
    break_targets: Vec<String>,
    continue_targets: Vec<String>,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            quads: Vec::new(),
            temp_counter: 0,
            label_counter: 0,
            break_targets: Vec::new(),
            continue_targets: Vec::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let s = format!("_t{}", self.temp_counter);
        self.temp_counter += 1;
        s
    }

    fn new_label(&mut self) -> String {
        let s = format!("_L{}", self.label_counter);
        self.label_counter += 1;
        s
    }

    fn emit(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    fn emit_label(&mut self, label: &str) {
        self.emit(Quad::new("label", "", "", label));
    }

    pub fn token_type_to_op(token_kind: &crate::token::TokenKind) -> &'static str {
        use crate::token::TokenKind::*;
        match token_kind {
            T_PLUS     => "+",
            T_MINUS    => "-",
            T_STAR     => "*",
            T_SLASH    => "/",
            T_PERCENT  => "%",
            T_LT       => "<",
            T_GT       => ">",
            T_LTE      => "<=",
            T_GTE      => ">=",
            T_EQUALSOP => "==",
            T_NEQ      => "!=",
            T_ANDAND   => "&&",
            T_OROR     => "||",
            T_NOT      => "not",
            _ => "op_unhandled",
        }
    }

    pub fn generate_expression(&mut self, expr: &Expr) -> String {
        use crate::parser::ast::Expr::*;
        match expr {
            Integer(val) => val.to_string(),
            Float(val) => val.to_string(),
            StringLit(val) => val.clone(),
            Boolean(b) => b.to_string(),
            Identifier(name) => name.clone(),
            Binary { left, operator, right } => 
                self.generate_binary_op(operator, left, right),
            Unary { operator, expr } =>
                self.generate_unary_op(operator, expr),
            Call { callee, args } => {
                let func_name = if let Expr::Identifier(ref name) = **callee {
                    name.clone()
                } else {
                    panic!("Unsupported call target (not identifier)")
                };
                for arg in args {
                    self.generate_expression(arg);
                }
                let temp = self.new_temp();
                self.emit(Quad::new("call", func_name.as_str(), "", temp.as_str()));
                temp
            }
            Grouping(inner) => self.generate_expression(inner),
        }
    }

    pub fn generate_binary_op(
        &mut self,
        op: &crate::token::TokenKind,
        left: &Expr,
        right: &Expr,
    ) -> String {
        let l = self.generate_expression(left);
        let r = self.generate_expression(right);
        let res = self.new_temp();
        let op_str = IRGenerator::token_type_to_op(op);
        self.emit(Quad::new(op_str, l.as_str(), r.as_str(), res.as_str()));
        res
    }

    pub fn generate_unary_op(&mut self, op: &crate::token::TokenKind, operand: &Expr) -> String {
        let val = self.generate_expression(operand);
        let res = self.new_temp();
        use crate::token::TokenKind::*;
        match op {
            T_MINUS => self.emit(Quad::new("neg", val.as_str(), "", res.as_str())),
            T_NOT => self.emit(Quad::new("not", val.as_str(), "", res.as_str())),
            _ => panic!("Unhandled unary operator in IR generation"),
        }
        res
    }

    pub fn generate_assignment(&mut self, ident: &str, value: &Expr) -> String {
        let val = self.generate_expression(value);
        self.emit(Quad::new("copy", val.as_str(), "", ident));
        ident.to_string()
    }

    pub fn generate_statement(&mut self, stmt: &Stmt) {
        use crate::parser::ast::Stmt::*;
        match stmt {
            Let { name, type_annot: _, value } => self.generate_var_decl(name, Some(value)),
            Expr(expr) => { self.generate_expression(expr); }
            Block(stmts) => for stmt in stmts { self.generate_statement(stmt); },
            If { condition, then_branch, else_branch } => {
                self.generate_if_stmt(condition, then_branch, else_branch.as_deref());
            }
            While { condition, body } => self.generate_while_stmt(condition, body),
            Break => self.generate_break_stmt(),
            Return(expr) => self.generate_return_stmt(expr.as_ref()),
            _ => {}
        }
    }

    pub fn generate_var_decl(&mut self, ident: &str, expr: Option<&Expr>) {
        if let Some(v) = expr {
            let val = self.generate_expression(v);
            self.emit(Quad::new("copy", val.as_str(), "", ident));
        }
    }

    pub fn generate_if_stmt(&mut self, condition: &Expr, then_stmt: &Stmt, else_stmt: Option<&Stmt>) {
        let cond = self.generate_expression(condition);
        let end_label = self.new_label();
        let else_label = if else_stmt.is_some() { self.new_label() } else { end_label.clone() };
        self.emit(Quad::new("if_false", cond.as_str(), "", else_label.as_str()));
        self.generate_statement(then_stmt);
        if let Some(es) = else_stmt {
            self.emit(Quad::new("goto", "", "", end_label.as_str()));
            self.emit_label(else_label.as_str());
            self.generate_statement(es);
        }
        self.emit_label(end_label.as_str());
    }

    pub fn generate_while_stmt(&mut self, condition: &Expr, body: &Stmt) {
        let loop_start = self.new_label();
        let loop_end = self.new_label();
        self.break_targets.push(loop_end.clone());
        self.continue_targets.push(loop_start.clone());
        self.emit_label(loop_start.as_str());
        let cond = self.generate_expression(condition);
        self.emit(Quad::new("if_false", cond.as_str(), "", loop_end.as_str()));
        self.generate_statement(body);
        self.emit(Quad::new("goto", "", "", loop_start.as_str()));
        self.emit_label(loop_end.as_str());
        self.continue_targets.pop();
        self.break_targets.pop();
    }

    pub fn generate_break_stmt(&mut self) {
        if let Some(target) = self.break_targets.last() {
            self.emit(Quad::new("goto", "", "", target.as_str()));
        }
    }

    pub fn generate_continue_stmt(&mut self) {
        if let Some(target) = self.continue_targets.last() {
            self.emit(Quad::new("goto", "", "", target.as_str()));
        }
    }

    pub fn generate_return_stmt(&mut self, expr: Option<&Expr>) {
        if let Some(e) = expr {
            let val = self.generate_expression(e);
            self.emit(Quad::new("return", val.as_str(), "", ""));
        } else {
            self.emit(Quad::new("return", "", "", ""));
        }
    }

    pub fn generate(&mut self, program: &[Stmt]) -> Vec<Quad> {
        self.quads.clear();
        self.temp_counter = 0;
        self.label_counter = 0;
        self.break_targets.clear();
        self.continue_targets.clear();
        for stmt in program {
            self.generate_statement(stmt);
        }
        self.quads.clone()
    }

    pub fn print_ir_code(&self) {
        println!("# Intermediate Representation (TAC)\n--- Generated Three-Address Code (TAC) ---");
        for quad in &self.quads {
            println!("{}", quad.to_string());
        }
        println!();
    }
}
