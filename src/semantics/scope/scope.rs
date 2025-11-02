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
    
    // pub fn push_scope(&mut self){

    // }
}