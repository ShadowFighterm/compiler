use std::collections::HashMap;

#[derive(Debug)]
pub enum ScopeError {
    UndeclaredVariableAccessed,
    UndefinedFunctionCalled,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
    FunctionRedefinition,
    FunctionRedefinitionAsPrototype,
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
        Scope {symbols: HashMap::new(), parent,
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack { // Spaghetti Stack
    pub current: Option<Box<Scope>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack { current: None }
    }

    pub fn push_scope(&mut self) {
        let new_scope = Scope::new(self.current.take()); // .take() returns the current original value, leaving None in its place
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
         
        if let Some(scope) = self.current_scope_mut() 
        //  attempting to unwrap an Option. If it contains a value, assign it to scope and run the block; otherwise skip the block.
        { 
            if scope.symbols.contains_key(&name) {
                //     new symbol kind  v/s existing symbol kind
                match (&symbol.kind, &scope.symbols[&name].kind) {
                    (SymbolKind::Variable, SymbolKind::Variable)  => {
                        return Err(ScopeError::VariableRedefinition)
                    }
                    (SymbolKind::Function { defined: false, .. },SymbolKind::Function { defined: false, .. })  => {
                        return Err(ScopeError::FunctionPrototypeRedefinition);
                    }
                    (SymbolKind::Function { defined: true, .. },SymbolKind::Function { defined: true, .. })  => {
                        return Err(ScopeError::FunctionRedefinition);
                    }
                    (SymbolKind::Function { defined: false, .. },SymbolKind::Function { defined: true, .. })  => {
                        return Err(ScopeError::FunctionRedefinitionAsPrototype);
                    }
                    _ => {} //Anything else is ignored
                }
            }
            scope.symbols.insert(name, symbol);
        }
        Ok(())
    }

    pub fn lookup_function(&self, name: &str) -> Result<&Symbol, ScopeError> {
        // begin search from this current scope

        let mut current = self.current.as_deref();
    
        // climb upward through parent scopes
        while let Some(scope) = current {
            // find the symbol in the current scope
            if let Some(sym) = scope.symbols.get(name) {
                match &sym.kind {

                    SymbolKind::Function { defined: true, .. } => {
                        return Ok(sym);
                    }
                    // found prototype
                    SymbolKind::Function { defined: false, .. } => {
                        return Err(ScopeError::UndefinedFunctionCalled);
                    }
                    // found something, but its not a function i.e., var()
                    _ => {
                    }
                }
            }
            // stepp up the ladder
            current = scope.parent.as_deref();
        }
    
        // symbol not found in any scope
        Err(ScopeError::UndefinedFunctionCalled)
    }
    
    pub fn lookup_variable(&self, name: &str) -> Result<&Symbol, ScopeError> {
        let mut current = self.current.as_deref();
    
        while let Some(scope) = current {
            if let Some(sym) = scope.symbols.get(name) {
    
                match &sym.kind {
                    // found 
                    SymbolKind::Variable => {
                        return Ok(sym);
                    }
    
                    // parameter found
                    SymbolKind::Parameter => {
                        return Ok(sym);
                    }
    
                    // found a function when expecting a variable 
                    SymbolKind::Function { .. } => {
                        return Err(ScopeError::UndeclaredVariableAccessed);
                    }
                }
            }
    
            current = scope.parent.as_deref();
        }
    
        Err(ScopeError::UndeclaredVariableAccessed)
    }
    
}
