use std::collections::HashMap;

#[derive(Debug)]
pub enum ScopeError{
    UndeclaredVariableAccessed,
    UndefinedFunctionCalled,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

#[derive(Debug,Clone)]
pub enum symbolKind{
    Variable,
    Function,
    Parameter,
}

#[derive(Debug,Clone)]
pub struct Symbol{
    pub name: String,
    pub kind: symbolKind,
    pub scope_level: usize,
    pub data_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Scope{
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope{
    pub fn new(parent: Option<Box<Scope>>)->Self{
        Scope{symbols: HashMap::new(),parent,}
    }
}

#[derive(Debug)]
pub struct ScopeStack{
    pub current: Option<Box<Scope>>,
}

impl ScopeStack{
    pub fn new()->Self{
        ScopeStack{ current: None }
    }
    
    pub fn push_scope(&mut self){
        let new_scope = Scope::new(self.current.take());
        self.current = Some(Box::new(new_scope)) 
    }

    pub fn pop_scope(&mut self){
        if let Some(cur) = self.current.take(){
            self.current = cur.parent;
        }
    }

    pub fn insert_symbol(&mut self, name: String, symbol: Symbol,) -> Result<(), ScopeError>
    {
        if let Some(curr) = self.current.as_mut() {
            if curr.symbols.contains_key(&name) {
                return Err(ScopeError::VariableRedefinition);
            }
            curr.symbols.insert(name, symbol);
            Ok(())
        } else {
            // No scope exists yet, create global
            self.push_scope();
            self.insert_symbol(name, symbol)
        }
    }
    pub fn lookup_symbol(&self, name: &str) -> Result<Symbol, ScopeError> {
        let mut scope_opt = self.current.as_ref();
        while let Some(scope) = scope_opt {
            if let Some(sym) = scope.symbols.get(name) {
                return Ok(sym.clone());
            }
            scope_opt = scope.parent.as_ref();
        }
        Err(ScopeError::UndeclaredVariableAccessed)
    }
}


