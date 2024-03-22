use std::{collections::LinkedList, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Colour,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Colour => write!(f, "colour"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Symbol {
    pub lexeme: String,
    pub r#type: Option<Type>,
}

impl Symbol {
    pub fn new(lexeme: &str, r#type: Option<Type>) -> Self {
        Symbol {
            lexeme: lexeme.to_string(),
            r#type,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: LinkedList<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: LinkedList::new(),
        }
    }

    pub fn token_to_type(&self, token: &str) -> Option<Type> {
        match token {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "bool" => Some(Type::Bool),
            "colour" => Some(Type::Colour),
            _ => None,
        }
    }

    pub fn add_symbol(&mut self, lexeme: &str, r#type: Option<Type>) {
        let mut index = 0;
        let symbol = Symbol::new(lexeme, r#type);
        for s in &self.symbols {
            if s < &symbol {
                break;
            }
            index += 1;
        }

        self.insert_at(index, symbol);
    }

    pub fn insert_at(&mut self, index: usize, symbol: Symbol) {
        let mut tail = self.symbols.split_off(index);
        self.symbols.push_back(symbol);
        self.symbols.append(&mut tail);
    }

    pub fn find_symbol(&self, symbol: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.lexeme == symbol)
    }

    pub fn set_type(&mut self, symbol: &str, r#type: Type) {
        if let Some(s) = self.symbols.iter_mut().find(|s| s.lexeme == symbol) {
            s.r#type = Some(r#type);
        }
    }
}
