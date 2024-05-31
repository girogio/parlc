use std::{collections::LinkedList, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Colour,
    Void,
    Array(Box<Type>, usize),
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Colour => write!(f, "colour"),
            Type::Array(t, size) => write!(f, "{}[{}]", t, size),
            Type::Void => write!(f, "void"),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SymbolType {
    Variable(Type),
    Function(Signature),
    Array(Type, usize),
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

#[derive(Debug, Copy, Clone)]
pub struct MemoryLocation {
    pub stack_level: usize,
    pub frame_index: usize,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub lexeme: String,
    pub symbol_type: SymbolType,
    pub memory_location: Option<MemoryLocation>,
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
    pub fn new(lexeme: &str, symbol_type: SymbolType, mem_loc: Option<MemoryLocation>) -> Self {
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

    pub fn add_symbol(
        &mut self,
        lexeme: &str,
        symbol_type: &SymbolType,
        mem_loc: Option<MemoryLocation>,
    ) {
        let symbol = Symbol::new(lexeme, symbol_type.clone(), mem_loc);

        // Find the index to insert the symbol
        let mut index = 0;
        for s in &self.symbols {
            if s >= &symbol {
                break;
            }
            index += 1;
        }

        self.insert_at(index, symbol);
    }

    /// Insert symbol at the given index
    pub fn insert_at(&mut self, index: usize, symbol: Symbol) {
        let mut tail = self.symbols.split_off(index);
        self.symbols.push_back(symbol);
        self.symbols.append(&mut tail);
    }

    pub fn find_symbol(&self, symbol: &str) -> Option<&Symbol> {
        // if self.symbols.is_empty() {
        //     return None;
        // }

        // let symbol_table_as_slice = self.symbols.iter().collect::<Vec<_>>();

        // symbol_table_as_slice
        //     .binary_search_by_key(&symbol, |s| &s.lexeme)
        //     .ok()
        //     .map(|index| symbol_table_as_slice[index])
        self.symbols.iter().find(|s| s.lexeme == symbol)
    }

    pub fn find_symbol_mut(&mut self, symbol: &str) -> Option<&mut Symbol> {
        self.symbols.iter_mut().find(|s| s.lexeme == symbol)
    }

    pub fn all_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn test_symbol_table() -> SymbolTable {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_symbol("d", &SymbolType::Variable(Type::Colour), None);
        symbol_table.add_symbol("a", &SymbolType::Variable(Type::Int), None);
        symbol_table.add_symbol("c", &SymbolType::Variable(Type::Bool), None);
        symbol_table.add_symbol("b", &SymbolType::Variable(Type::Float), None);
        symbol_table.add_symbol(
            "e",
            &SymbolType::Array(Type::Int, 10),
            Some(MemoryLocation {
                stack_level: 0,
                frame_index: 0,
            }),
        );
        symbol_table
    }

    #[rstest]
    fn check_table_ord() {
        let symbol_table = test_symbol_table();

        for (i, symbol) in symbol_table.symbols.iter().enumerate() {
            println!("{}: {}", i, symbol.lexeme);
        }
    }

    #[rstest]
    fn check_insert_at() {
        let mut symbol_table = test_symbol_table();

        symbol_table.insert_at(
            2,
            Symbol::new(
                "f",
                SymbolType::Variable(Type::Array(Box::new(Type::Float), 10)),
                Some(MemoryLocation {
                    stack_level: 0,
                    frame_index: 0,
                }),
            ),
        );

        assert_eq!(symbol_table.symbols.len(), 6);
        assert_eq!(symbol_table.symbols.iter().nth(2).unwrap().lexeme, "f");
    }
}
