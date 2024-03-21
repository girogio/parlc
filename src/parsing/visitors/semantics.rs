use std::{borrow::Borrow, collections::LinkedList};

use crate::parsing::ast::{AstNode, Visitor};
use crate::utils::errors::{Error, SemanticError};
use crate::utils::Result;

#[derive(Debug)]
struct SymbolTable {
    symbols: LinkedList<String>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            symbols: LinkedList::new(),
        }
    }

    fn add_symbol(&mut self, symbol: String) {
        let mut index = 0;
        for s in &self.symbols {
            if s > &symbol {
                break;
            }
            index += 1;
        }

        self.insert_at(index, symbol);
    }

    fn insert_at(&mut self, index: usize, symbol: String) {
        let mut tail = self.symbols.split_off(index);
        self.symbols.push_back(symbol);
        self.symbols.append(&mut tail);
    }

    fn find_symbol(&self, symbol: &str) -> Option<&String> {
        self.symbols.iter().find(|s| s == &symbol)
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

    fn find_symbol(&self, symbol: &str) -> Option<&String> {
        self.symbol_table
            .iter()
            .rev()
            .find_map(|table| table.find_symbol(symbol))
    }

    fn add_symbol(&mut self, symbol: String) {
        self.symbol_table.last_mut().unwrap().add_symbol(symbol);
    }

    fn check_scope(&self, symbol: &str) -> bool {
        self.symbol_table
            .iter()
            .rev()
            .any(|table| table.find_symbol(symbol).is_some())
    }
}

impl Visitor for SemanticAnalyzer {
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

            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                self.visit(identifier)?;
                Ok(())
            }

            AstNode::Assignment {
                identifier,
                expression,
            } => {
                if let AstNode::Identifier { token } = identifier.borrow() {
                    self.add_symbol(token.span.lexeme.clone());
                }
                self.visit(expression)?;
                Ok(())
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                if let AstNode::Identifier { token } = identifier.borrow() {
                    self.add_symbol(token.span.lexeme.clone());
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
                var_type,
                expression,
            } => {
                if let AstNode::Identifier { token } = identifier.borrow() {
                    self.add_symbol(token.span.lexeme.clone());
                }
                self.visit(expression)?;

                Ok(())
            }

            AstNode::Expression {
                casted_type,
                bin_op,
            } => {
                // self.visit(casted_type);
                self.visit(bin_op)?;
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
                if self.check_scope(&token.span.lexeme) {
                    return Err(SemanticError::UndefinedVariable(token.span.lexeme.clone()).into());
                }
                Ok(())
            }

            AstNode::UnaryOp { operator, expr } => todo!(),
            AstNode::PadWidth => todo!(),
            AstNode::PadRandI { upper_bound } => todo!(),
            AstNode::PadHeight => todo!(),
            AstNode::PadRead { first, second } => todo!(),
            AstNode::IntLiteral(_) => todo!(),
            AstNode::FloatLiteral(_) => todo!(),
            AstNode::BoolLiteral(_) => todo!(),
            AstNode::ColourLiteral(_) => todo!(),
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
