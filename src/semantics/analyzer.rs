use std::collections::HashMap;

pub enum Symbol {
    Scope,
    Variable,
    Function,
}

struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    fn push_scope(&mut self) {
        self.symbols.insert("".to_string(), Symbol::Scope);
    }

    fn pop_scope(&mut self) {
        self.symbols.remove("");
    }

    fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
