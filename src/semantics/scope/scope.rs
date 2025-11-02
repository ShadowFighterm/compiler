use std::collections::HashMap;

#[derive(Debug)]
pub enum ScopeError {
    UndeclaredVariableAccessed,
    UndefinedFunctionCalled,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable,
    Function { params: Vec<String>, defined: bool },
    Parameter,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope_level: usize,
    pub symbol_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack {
    pub current: Option<Box<Scope>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack { current: None }
    }

    pub fn push_scope(&mut self) {
        let new_scope = Scope::new(self.current.take());
        self.current = Some(Box::new(new_scope))
    }

    pub fn pop_scope(&mut self) {
        if let Some(cur) = self.current.take() {
            self.current = cur.parent;
        }
    }

    fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.current.as_deref_mut()
    }

    pub fn insert_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), ScopeError> {
        if let Some(scope) = self.current_scope_mut() {
            if scope.symbols.contains_key(&name) {
                match &symbol.kind {
                    SymbolKind::Variable => return Err(ScopeError::VariableRedefinition),
                    SymbolKind::Function { defined: false, .. } => {
                        return Err(ScopeError::FunctionPrototypeRedefinition);
                    }
                    _ => {}
                }
            }
            scope.symbols.insert(name, symbol);
        }
        Ok(())
    }

    pub fn lookup_function(&self, name: &str) -> Result<&Symbol, ScopeError> {
        let mut current = self.current.as_deref();

        while let Some(scope) = current {
            if let Some(sym) = scope.symbols.get(name) {
                if let SymbolKind::Function { defined, .. } = &sym.kind {
                    if *defined {
                        return Ok(sym);
                    } else {
                        return Err(ScopeError::UndefinedFunctionCalled);
                    }
                }
            }
            current = scope.parent.as_deref();
        }

        Err(ScopeError::UndefinedFunctionCalled)
    }
}
