use std::collections::LinkedList;
use std::fmt::Display;

use crate::utils::Result;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};
use crate::{core::TokenKind, utils::errors::SemanticError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Type {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Symbol {
    lexeme: String,
    r#type: Type,
}

impl Symbol {
    fn new(lexeme: &str, r#type: Type) -> Self {
        Symbol {
            lexeme: lexeme.to_string(),
            r#type,
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    symbols: LinkedList<Symbol>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            symbols: LinkedList::new(),
        }
    }

    fn add_symbol(&mut self, lexeme: &str, r#type: Type) {
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

    fn insert_at(&mut self, index: usize, symbol: Symbol) {
        let mut tail = self.symbols.split_off(index);
        self.symbols.push_back(symbol);
        self.symbols.append(&mut tail);
    }

    fn find_symbol(&self, symbol: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.lexeme == symbol)
    }
}

#[derive(Debug)]
pub struct SemanticAnalyzer {
    symbol_table: Vec<SymbolTable>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: Vec::new(),
        }
    }

    fn find_symbol(&self, symbol: &Token) -> Option<&Symbol> {
        self.symbol_table
            .iter()
            .rev()
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
    }

    fn add_symbol(&mut self, symbol: &Token, r#type: &Token) -> Result<()> {
        let computed_type = self.compute_type(r#type)?;
        self.symbol_table
            .last_mut()
            .unwrap()
            .add_symbol(&symbol.span.lexeme, computed_type);
        Ok(())
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.symbol_table
            .iter()
            .rev()
            .any(|table| table.find_symbol(&symbol.span.lexeme).is_some())
    }

    fn compute_type(&self, symbol: &Token) -> Result<Type> {
        match symbol.kind {
            TokenKind::IntLiteral => Ok(Type::Int),
            TokenKind::FloatLiteral => Ok(Type::Float),
            TokenKind::BoolLiteral => Ok(Type::Bool),
            TokenKind::ColourLiteral => Ok(Type::Colour),
            TokenKind::Identifier => {
                if let Some(s) = self.find_symbol(symbol) {
                    Ok(s.r#type)
                } else {
                    Err(SemanticError::UndefinedVariable(symbol.clone()).into())
                }
            }
            _ => Err(SemanticError::TypeMismatch(
                symbol.clone(),
                "int, float, bool or colour".to_string(),
            )
            .into()),
        }
    }

    fn assert_type(&self, symbol: &Token, expected: Type) -> Result<()> {
        if let Some(s) = self.find_symbol(symbol) {
            if s.r#type == expected {
                Ok(())
            } else {
                Err(SemanticError::TypeMismatch(symbol.clone(), expected.to_string()).into())
            }
        } else {
            Err(SemanticError::UndefinedVariable(symbol.clone()).into())
        }
    }
}

impl Visitor<()> for SemanticAnalyzer {
    fn visit(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program { statements } => {
                self.symbol_table.push(SymbolTable::new());
                for statement in statements {
                    self.visit(statement)?;
                }
                self.symbol_table.pop();
                Ok(())
            }

            AstNode::Block { statements } => {
                self.symbol_table.push(SymbolTable::new());
                for statement in statements {
                    self.visit(statement)?;
                }
                self.symbol_table.pop();
                Ok(())
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                if self.check_scope(identifier) {
                    return Err(SemanticError::AlreadyDefinedFunction(identifier.clone()).into());
                } else {
                    self.add_symbol(identifier, return_type)?;
                }

                self.symbol_table.push(SymbolTable::new());
                for param in params {
                    self.visit(param)?;
                }
                self.visit(block)?;
                self.symbol_table.pop();
                Ok(())
            }

            AstNode::VarDec {
                identifier,
                r#type: var_type,
                expression,
            } => {
                if self.check_scope(identifier) {
                    return Err(SemanticError::AlreadyDefinedVariable(identifier.clone()).into());
                } else {
                    self.add_symbol(identifier, var_type)?;
                }

                self.visit(expression)?;

                Ok(())
            }

            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                if self.check_scope(identifier) {
                    return Err(SemanticError::RedeclaredVariable(identifier.clone()).into());
                } else {
                    self.add_symbol(identifier, param_type)?;
                }
                Ok(())
            }

            AstNode::Expression {
                casted_type,
                bin_op,
            } => {
                self.visit(bin_op)?;
                Ok(())
            }

            AstNode::SubExpression { bin_op } => {
                self.visit(bin_op)?;
                Ok(())
            }

            AstNode::Assignment {
                identifier,
                expression,
            } => {
                // self.visit(identifier)?;

                self.visit(expression)?;
                Ok(())
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                self.visit(left)?;
                self.visit(right)?;
                Ok(())
            }

            AstNode::Identifier { token } => {
                if self.find_symbol(token).is_none() {
                    return Err(SemanticError::UndefinedVariable(token.clone()).into());
                }
                Ok(())
            }

            AstNode::UnaryOp { operator, expr } => {
                self.visit(expr)?;
                Ok(())
            }
            AstNode::PadWidth => Ok(()),
            AstNode::PadRandI { upper_bound } => todo!(),
            AstNode::PadHeight => todo!(),
            AstNode::PadRead { first, second } => todo!(),
            AstNode::IntLiteral(_) => Ok(()),
            AstNode::FloatLiteral(_) => Ok(()),
            AstNode::BoolLiteral(_) => Ok(()),
            AstNode::ColourLiteral(_) => Ok(()),
            AstNode::FunctionCall { identifier, args } => todo!(),
            AstNode::ActualParams { params } => {
                for param in params {
                    self.visit(param)?;
                }
                Ok(())
            }
            AstNode::Delay { expression } => todo!(),
            AstNode::Return { expression } => {
                self.visit(expression)?;
                Ok(())
            }
            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => todo!(),
            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => todo!(),
            AstNode::If {
                condition,
                if_true,
                if_false,
            } => todo!(),
            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => todo!(),
            AstNode::While { condition, body } => todo!(),
            AstNode::Print { expression } => todo!(),
            AstNode::PadClear { expr } => {
                self.visit(expr)?;
                Ok(())
            }
            AstNode::EndOfFile => todo!(),
        }
    }
}
