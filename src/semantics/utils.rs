use std::{collections::LinkedList, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Colour,
    Void,
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Colour => write!(f, "colour"),
            Type::Void => write!(f, "void"),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SymbolType {
    Variable(Type),
    Function(Signature),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Signature {
    pub parameters: Vec<(Type, String)>,
    pub return_type: Type,
    pub instruction_index: Option<usize>,
}

impl Signature {
    pub fn new(return_type: Type) -> Self {
        Signature {
            parameters: Vec::new(),
            return_type,
            instruction_index: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemLoc {
    pub stack_level: usize,
    pub frame_index: usize,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub lexeme: String,
    pub symbol_type: SymbolType,
    pub memory_location: Option<MemLoc>,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.lexeme == other.lexeme
    }
}

impl Eq for Symbol {}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.lexeme.cmp(&other.lexeme))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lexeme.cmp(&other.lexeme)
    }
}

impl Symbol {
    pub fn new(lexeme: &str, symbol_type: SymbolType, mem_loc: Option<MemLoc>) -> Self {
        Symbol {
            lexeme: lexeme.to_string(),
            symbol_type,
            memory_location: mem_loc,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: LinkedList<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: LinkedList::new(),
        }
    }

    pub fn token_to_type(&self, token: &str) -> Type {
        match token {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "colour" => Type::Colour,
            _ => unreachable!(),
        }
    }

    pub fn add_symbol(&mut self, lexeme: &str, symbol_type: &SymbolType, mem_loc: Option<MemLoc>) {
        let mut index = 0;
        let symbol = Symbol::new(lexeme, symbol_type.clone(), mem_loc);
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

    pub fn all_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter()
    }
}
