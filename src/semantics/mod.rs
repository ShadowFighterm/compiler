pub mod scope;
// pub mod typechecker; // comment out to avoid compilation errors

pub use scope::scope::{ScopeStack, Symbol, SymbolKind, Type, ScopeError};
