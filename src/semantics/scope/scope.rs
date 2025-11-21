use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type { //Had to introduce Type enum to make function signatures work
    Int,
    Float,
    Bool,
    Char,
    String,
    Void,
    Unknown,
    Custom(String),
    Array(Box<Type>, usize),
    Pointer(Box<Type>),
}

#[derive(Debug)]
pub enum ScopeError {
    // name resolution / kind mismatch
    UndeclaredIdentifier,
    FoundButWrongKind,              
    // functions
    UndefinedFunctionCalled,        // only prototype found (or not found)
    FunctionPrototypeRedefinition,
    FunctionRedefinition,
    FunctionRedefinitionAsPrototype,
    FunctionSignatureConflict,
    // variables
    VariableRedefinition,
    VariableUsedBeforeInit,
    // generic
    NoCurrentScope,
    BreakMustInsideLoop,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable { mutable: bool }, 
    Parameter,
    Function {
        params: Vec<Type>,
        return_type: Type,
        defined: bool, 
    },
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope_level: usize,
    pub ty: Option<Type>,        
    pub initialized: bool,       
}

impl Symbol {
    pub fn new_variable(name: impl Into<String>, ty: Type, mutable: bool, scope_level: usize, initialized: bool) -> Self {
        Self {
            name: name.into(),
            kind: SymbolKind::Variable { mutable },
            scope_level,
            ty: Some(ty),
            initialized,
        }
    }

    // Example usage: 
    // let p1 = new_parameter("x", ty, 0);        // &str
    // let p2 = new_parameter(String::from("y"), ty, 0); // String

    pub fn new_parameter(name: impl Into<String>, ty: Type, scope_level: usize) -> Self {
        Self {
            name: name.into(),
            kind: SymbolKind::Parameter,
            scope_level,
            ty: Some(ty),
            initialized: true, // parameters are considered initialized
        }
    }

    pub fn new_function_prototype(name: impl Into<String>, params: Vec<Type>, return_type: Type, scope_level: usize) -> Self {
        Self {
            name: name.into(),
            kind: SymbolKind::Function { params, return_type: return_type.clone(), defined: false },
            scope_level,
            ty: Some(return_type),
            initialized: true,
        }
    }

    pub fn new_function_definition(name: impl Into<String>, params: Vec<Type>, return_type: Type, scope_level: usize) -> Self {
        Self {
            name: name.into(),
            kind: SymbolKind::Function { params, return_type: return_type.clone(), defined: true },
            scope_level,
            ty: Some(return_type),
            initialized: true,
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
    pub level: usize,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {

        let level = if let Some(ref p) = parent {
            p.level + 1
        } else {
            0
        };

        Scope {
            symbols: HashMap::new(),
            parent,
            level,
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack {
    pub current: Option<Box<Scope>>,
    loop_depth: usize,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack { current: None, loop_depth:0 }
    }

    pub fn enter_loop(&mut self) {
        self.loop_depth += 1;
    }

    pub fn exit_loop(&mut self) {
        self.loop_depth -= 1;
    }

    pub fn in_loop(&self) -> bool {
        self.loop_depth > 0
    }

    /// Enter a new inner scope
    pub fn enter_scope(&mut self) {
        let new_scope = Scope::new(self.current.take()); // take ownership of the current Option<Box<Scope>>, leave self.current as None
        self.current = Some(Box::new(new_scope));
    }

    /// exit current scope and return to parent
    pub fn exit_scope(&mut self) {
        if let Some(cur) = self.current.take() {
            self.current = cur.parent;
        }
    }

    fn current_scope_mut(&mut self) -> Result<&mut Scope, ScopeError> {
        self.current.as_deref_mut().ok_or(ScopeError::NoCurrentScope)
    }

    fn current_scope(&self) -> Option<&Scope> {
        self.current.as_deref()
    }

    /// Insert a variable into the current scope.
    pub fn insert_variable(&mut self, name: String, ty: Type, mutable: bool, initialized: bool) -> Result<(), ScopeError> {
        let scope = self.current_scope_mut()?;
        if scope.symbols.contains_key(&name) {

            return Err(ScopeError::VariableRedefinition);
        }
        let sym = Symbol::new_variable(name.clone(), ty, mutable, scope.level, initialized);
        scope.symbols.insert(name, sym);
        Ok(())
    }

 
    pub fn insert_function_prototype(&mut self, name: String, params: Vec<Type>, return_type: Type) -> Result<(), ScopeError> {
        let scope = self.current_scope_mut()?;
        if let Some(existing) = scope.symbols.get(&name) {
            match &existing.kind {
                SymbolKind::Function { defined: false, params: existing_params, return_type: existing_ret, .. } => {
                    
                    if &existing_params[..] == &params[..] && existing_ret == &return_type {
                        return Err(ScopeError::FunctionPrototypeRedefinition);
                    } else {
                        return Err(ScopeError::FunctionSignatureConflict);
                    }
                }
                SymbolKind::Function { defined: true, .. } => {
                    return Err(ScopeError::FunctionRedefinitionAsPrototype);
                }
                _ => {
                    return Err(ScopeError::VariableRedefinition); // name clash with variable/param
                }
            }
        }
        let sym = Symbol::new_function_prototype(name.clone(), params, return_type, scope.level);
        scope.symbols.insert(name, sym);
        Ok(())
    }

    pub fn insert_function_definition(&mut self, name: String, params: Vec<Type>, return_type: Type) -> Result<(), ScopeError> {
        let scope = self.current_scope_mut()?;
        if let Some(existing) = scope.symbols.get(&name) {
            match &existing.kind {
                SymbolKind::Function { defined: false, params: existing_params, return_type: existing_ret, .. } => {
                    
                    if &existing_params[..] != &params[..] || existing_ret != &return_type {
                        return Err(ScopeError::FunctionSignatureConflict);
                    } else {

                        let sym = Symbol::new_function_definition(name.clone(), params, return_type, scope.level);
                        scope.symbols.insert(name, sym);
                        return Ok(());
                    }
                }
                SymbolKind::Function { defined: true, .. } => {
                    return Err(ScopeError::FunctionRedefinition);
                }
                _ => {
                    return Err(ScopeError::VariableRedefinition); // name clash
                }
            }
        }

        let sym = Symbol::new_function_definition(name.clone(), params, return_type, scope.level);
        scope.symbols.insert(name, sym);
        Ok(())
    }


    fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current.as_deref();
        while let Some(scope) = current {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
            current = scope.parent.as_deref(); //At each step, scope is a reference to the current scope in the chain
        }
        None
    }


    pub fn lookup_variable(&self, name: &str) -> Result<&Symbol, ScopeError> {
        if let Some(sym) = self.find_symbol(name) {
            match &sym.kind {
                SymbolKind::Variable { .. } | SymbolKind::Parameter => {
                    if !sym.initialized {
                        return Err(ScopeError::VariableUsedBeforeInit);
                    }
                    return Ok(sym);
                }
                SymbolKind::Function { .. } => {
                    return Err(ScopeError::FoundButWrongKind);
                }
            }
        }
        Err(ScopeError::UndeclaredIdentifier)
    }


    pub fn lookup_function(&self, name: &str) -> Result<&Symbol, ScopeError> {
        if let Some(sym) = self.find_symbol(name) {
            match &sym.kind {
                SymbolKind::Function { defined: true, .. } => return Ok(sym),
                SymbolKind::Function { defined: false, .. } => return Err(ScopeError::UndefinedFunctionCalled),
                _ => return Err(ScopeError::FoundButWrongKind),
            }
        }
        Err(ScopeError::UndefinedFunctionCalled)
    }


    pub fn lookup_symbol_any(&self, name: &str) -> Option<&Symbol> {
        self.find_symbol(name)
    }

    // this function is specifically intended for variables and parameters, not for functions
    pub fn mark_initialized(&mut self, name: &str) -> Result<(), ScopeError> {
        // Need to find the symbol mutably in the chain of scopes
        let mut current = self.current.as_deref_mut();
        while let Some(scope) = current {
            if let Some(sym) = scope.symbols.get_mut(name) {
                match sym.kind {
                    SymbolKind::Variable { .. } | SymbolKind::Parameter => {
                        sym.initialized = true;
                        return Ok(());
                    }
                    SymbolKind::Function { .. } => return Err(ScopeError::FoundButWrongKind),
                }
            }
            current = scope.parent.as_deref_mut();
        }
        Err(ScopeError::UndeclaredIdentifier)
    }


    // Check whether a variable or parameter with the given name exists in any scope
    pub fn variable_exists(&self, name: &str) -> bool {
        if let Some(symbol) = self.find_symbol(name) {
            // return true only if it is a var
            matches!(symbol.kind, SymbolKind::Variable { .. } | SymbolKind::Parameter)
        } else {
            false
        }
    }


    pub fn current_level(&self) -> usize {

        if let Some(scope) = self.current.as_deref() {
            scope.level
        } else {
            0
        }
    }

    
}

