use crate::semantics::utils::{Symbol, SymbolTable, Type};
use crate::utils::errors::SemanticError;
use crate::utils::Result;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};

#[derive(Debug)]
pub struct ScopeChecker {
    symbol_table: Vec<SymbolTable>,
    inside_function: bool,
}

impl ScopeChecker {
    pub fn new() -> Self {
        ScopeChecker {
            symbol_table: Vec::new(),
            inside_function: false,
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
            .nth(1)
            .unwrap()
            .find_symbol(&symbol.span.lexeme)
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
                self.inside_function = true;
                self.visit(block)?;
                self.symbol_table.pop();
                self.inside_function = false;
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
                self.add_symbol(identifier, param_type)?;
                Ok(())
            }

            AstNode::Expression {
                casted_type: _,
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
                identifier: _,
                expression,
            } => {
                // self.visit(identifier)?;

                self.visit(expression)?;
                Ok(())
            }

            AstNode::BinOp {
                left,
                operator: _,
                right,
            } => {
                self.visit(left)?;
                self.visit(right)?;
                Ok(())
            }

            AstNode::Identifier { token } => {
                if self.inside_function {
                    if !self.check_parent_scope(token) {
                        return Err(SemanticError::UndefinedVariable(token.clone()).into());
                    }
                } else if self.find_symbol(token).is_none() {
                    return Err(SemanticError::UndefinedVariable(token.clone()).into());
                }
                Ok(())
            }

            AstNode::UnaryOp { operator: _, expr } => {
                self.visit(expr)?;
                Ok(())
            }
            AstNode::PadWidth => Ok(()),
            AstNode::PadRandI { upper_bound: _ } => todo!(),
            AstNode::PadHeight => todo!(),
            AstNode::PadRead {
                first: _,
                second: _,
            } => todo!(),
            AstNode::IntLiteral(_) => Ok(()),
            AstNode::FloatLiteral(_) => Ok(()),
            AstNode::BoolLiteral(_) => Ok(()),
            AstNode::ColourLiteral(_) => Ok(()),
            AstNode::ActualParams { params } => {
                for param in params {
                    self.visit(param)?;
                }
                Ok(())
            }
            AstNode::Delay { expression: _ } => todo!(),
            AstNode::Return { expression } => {
                self.visit(expression)?;
                Ok(())
            }
            AstNode::PadWriteBox {
                loc_x: _,
                loc_y: _,
                width: _,
                height: _,
                colour: _,
            } => todo!(),
            AstNode::PadWrite {
                loc_x: _,
                loc_y: _,
                colour: _,
            } => todo!(),
            AstNode::If {
                condition: _,
                if_true: _,
                if_false: _,
            } => todo!(),
            AstNode::For {
                initializer: _,
                condition: _,
                increment: _,
                body: _,
            } => todo!(),
            AstNode::While {
                condition: _,
                body: _,
            } => todo!(),
            AstNode::Print { expression: _ } => todo!(),
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
    use crate::{
        core::{TextSpan, TokenKind},
        lexing::Lexer,
        parsing::Parser,
        utils::SimpleBuffer,
    };

    use super::*;
    use std::path::Path;

    fn check_semantics(input: &str) -> Result<()> {
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input, Path::new(""), None);

        let tokens = lexer.lex().unwrap();

        let mut parser = Parser::new(&tokens, Path::new(""));
        let ast = parser.parse()?;

        let mut scope_checker = ScopeChecker::new();
        scope_checker.visit(ast)?;

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
        let mut scope_checker = ScopeChecker::new();

        let token = Token::new(TokenKind::Identifier, TextSpan::new(0, 0, 0, 0, "asd"));

        assert!(!scope_checker.check_scope(&token));

        scope_checker
            .add_symbol(
                &token,
                &Token::new(TokenKind::Type, TextSpan::new(0, 0, 0, 0, "int")),
            )
            .unwrap();

        assert!(scope_checker.check_scope(&token));

        assert!(!scope_checker.check_parent_scope(&token));
    }
}
