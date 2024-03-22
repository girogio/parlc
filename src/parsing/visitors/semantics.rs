use std::collections::LinkedList;
use std::fmt::Display;

use crate::utils::errors::SemanticError;
use crate::utils::Result;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};

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
    r#type: Option<Type>,
}

impl Symbol {
    fn new(lexeme: &str, r#type: Option<Type>) -> Self {
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

    fn add_symbol(&mut self, lexeme: &str, r#type: Option<Type>) {
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

    fn set_type(&mut self, symbol: &str, r#type: Type) {
        if let Some(s) = self.symbols.iter_mut().find(|s| s.lexeme == symbol) {
            s.r#type = Some(r#type);
        }
    }
}

#[derive(Debug)]
pub struct ScopeChecker {
    symbol_table: Vec<SymbolTable>,
}

impl ScopeChecker {
    pub fn new() -> Self {
        ScopeChecker {
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
        self.symbol_table
            .last_mut()
            .unwrap()
            .add_symbol(&symbol.span.lexeme, None);

        self.symbol_table
            .last_mut()
            .unwrap()
            .set_type(&symbol.span.lexeme, token_type(r#type));
        Ok(())
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.symbol_table
            .iter()
            .last()
            .unwrap()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn check_parent_scope(&self, symbol: &Token) -> bool {
        self.symbol_table
            .iter()
            .rev()
            .skip(1)
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
            .is_some()
    }
}

fn token_type(r#type: &Token) -> Type {
    match r#type.span.lexeme.as_str() {
        "int" => Type::Int,
        "float" => Type::Float,
        "bool" => Type::Bool,
        "colour" => Type::Colour,
        _ => unreachable!(),
    }
}

impl Visitor<()> for ScopeChecker {
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
                dbg!(&self.symbol_table);
                self.symbol_table.pop();
                Ok(())
            }

            AstNode::VarDec {
                identifier,
                r#type: var_type,
                expression,
            } => {
                self.visit(expression)?;

                if self.check_scope(identifier) {
                    return Err(SemanticError::AlreadyDefinedVariable(identifier.clone()).into());
                } else {
                    self.add_symbol(identifier, var_type)?;
                }

                Ok(())
            }

            AstNode::FunctionCall { identifier, args } => {
                if self.find_symbol(identifier).is_none() {
                    return Err(SemanticError::UndefinedFunction(identifier.clone()).into());
                }

                for arg in args {
                    self.visit(arg)?;
                }

                Ok(())
            }

            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                if self.check_parent_scope(identifier) {
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

#[cfg(test)]
mod tests {
    use crate::{core::TextSpan, lexing::Lexer, parsing::Parser, utils::SimpleBuffer};

    use super::*;
    use std::path::Path;

    fn check_semantics(input: &str) -> Result<()> {
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input, &Path::new(""), None);

        let tokens = lexer.lex().unwrap();

        let mut parser = Parser::new(&tokens, &Path::new(""));
        let ast = parser.parse()?;

        let mut scope_checker = ScopeChecker::new();
        scope_checker.visit(&ast)?;

        Ok(())
    }

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();

        symbol_table.add_symbol("x", Some(Type::Int));
        symbol_table.add_symbol("y", Some(Type::Float));
        symbol_table.add_symbol("z", Some(Type::Bool));

        assert_eq!(
            symbol_table.find_symbol("x").unwrap().r#type,
            Some(Type::Int)
        );
        assert_eq!(
            symbol_table.find_symbol("y").unwrap().r#type,
            Some(Type::Float)
        );
        assert_eq!(
            symbol_table.find_symbol("z").unwrap().r#type,
            Some(Type::Bool)
        );
    }

    #[test]
    fn test_scope_checker() {
        let symbol_table = SymbolTable::new();

        let mut scope_checker = ScopeChecker {
            symbol_table: vec![symbol_table],
        };

        let token = Token::new(TokenKind::Identifier, TextSpan::new(0, 0, 0, 0, "asd"));

        assert!(!scope_checker.check_scope(&token));

        scope_checker
            .add_symbol(
                &token,
                &Token::new(TokenKind::Type, TextSpan::new(0, 0, 0, 0, "int")),
            )
            .unwrap();

        assert!(scope_checker.check_scope(&token));

        assert_eq!(scope_checker.check_parent_scope(&token), false);
    }
}
