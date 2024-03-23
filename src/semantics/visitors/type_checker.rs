use crate::semantics::utils::{Signature, Symbol, SymbolTable, SymbolType};
use crate::utils::errors::SemanticError;
use crate::utils::Result;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};

#[derive(Debug)]
pub struct TypeChecker {
    /// Stack of symbol tables, each representing a scope
    symbol_table: Vec<SymbolTable>,
    /// Flag to denote that the current scope lies within a function
    inside_function: bool,
    /// If this is 0, we can check for the existence of the symbol in any
    /// scope, up to the global scope.
    scope_peek_limit: usize,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: Vec::new(),
            inside_function: false,
            scope_peek_limit: 0,
        }
    }

    fn find_symbol(&self, symbol: &Token) -> Option<&Symbol> {
        self.symbol_table
            .iter()
            .rev()
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
    }

    fn current_scope(&self) -> &SymbolTable {
        self.symbol_table.last().unwrap()
    }

    fn mut_current_scope(&mut self) -> &mut SymbolTable {
        self.symbol_table.last_mut().unwrap()
    }

    fn add_symbol(&mut self, symbol: &Token, symbol_type: &SymbolType) -> Result<()> {
        self.mut_current_scope()
            .add_symbol(&symbol.span.lexeme, symbol_type);

        Ok(())
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.current_scope()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn push_scope(&mut self) {
        self.symbol_table.push(SymbolTable::new());
    }

    fn pop_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn check_up_to_scope(&self, symbol: &Token) -> bool {
        self.symbol_table
            .iter()
            .skip(self.scope_peek_limit)
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
            .is_some()
    }
}

impl Visitor<()> for TypeChecker {
    fn visit(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program { statements } => {
                self.push_scope();
                for statement in statements {
                    self.visit(statement)?;
                }
                self.pop_scope();
                Ok(())
            }

            AstNode::Block { statements } => {
                self.push_scope();
                for statement in statements {
                    self.visit(statement)?;
                }
                self.pop_scope();
                Ok(())
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                // Check that function name isn't already defined
                if self.check_scope(identifier) {
                    return Err(SemanticError::AlreadyDefinedFunction(identifier.clone()).into());
                }

                self.push_scope();
                self.scope_peek_limit = self.symbol_table.len() - 1;

                // Add the parameter symbols to the symbol table in this scope
                for param in params {
                    self.visit(param)?;
                }

                // all the parameters are added to the symbol table
                // now we add them to the function signature
                let mut signature =
                    Signature::new(self.current_scope().token_to_type(&return_type.span.lexeme));

                for param in self.current_scope().all_symbols() {
                    if let SymbolType::Variable(t) = param.symbol_type {
                        signature.parameters.push(t);
                    }
                }

                // set the signature of the function in the symbol table
                // add the function symbol to the previous scope to support recursion
                self.symbol_table
                    .iter_mut()
                    .rev()
                    .nth(1)
                    .unwrap()
                    .add_symbol(
                        &identifier.span.lexeme,
                        &SymbolType::Function(signature.clone()),
                    );

                self.inside_function = true;
                self.visit(block)?;
                self.symbol_table.pop();

                self.inside_function = false;
                self.scope_peek_limit = 0;

                Ok(())
            }

            AstNode::Identifier { token } => {
                if self.inside_function {
                    if !self.check_up_to_scope(token) {
                        dbg!(&self.symbol_table);
                        return Err(SemanticError::UndefinedVariable(token.clone()).into());
                    }
                } else if !self.check_scope(token) {
                    return Err(SemanticError::UndefinedVariable(token.clone()).into());
                }
                Ok(())
            }

            AstNode::VarDec {
                identifier,
                r#type: var_type,
                expression,
            } => {
                self.visit(expression)?;

                if self.check_scope(identifier) {
                    return Err(SemanticError::VariableRedaclaration(identifier.clone()).into());
                } else {
                    self.add_symbol(
                        identifier,
                        &SymbolType::Variable(
                            self.current_scope().token_to_type(&var_type.span.lexeme),
                        ),
                    )?;
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
                self.add_symbol(
                    identifier,
                    &SymbolType::Variable(
                        self.current_scope().token_to_type(&param_type.span.lexeme),
                    ),
                )?;
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
                identifier,
                expression,
            } => {
                if self.inside_function {
                    if !self.check_up_to_scope(identifier) {
                        return Err(SemanticError::UndefinedVariable(identifier.clone()).into());
                    }
                } else if !self.check_scope(identifier) {
                    return Err(SemanticError::UndefinedVariable(identifier.clone()).into());
                }

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

            AstNode::UnaryOp { operator: _, expr } => {
                self.visit(expr)?;
                Ok(())
            }
            AstNode::PadWidth => Ok(()),
            AstNode::PadRandI { upper_bound } => {
                self.visit(upper_bound)?;
                Ok(())
            }
            AstNode::PadHeight => Ok(()),
            AstNode::PadRead { first, second } => {
                self.visit(first)?;
                self.visit(second)?;
                Ok(())
            }
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
            AstNode::Delay { expression } => {
                self.visit(expression)?;
                Ok(())
            }
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
            } => {
                self.visit(loc_x)?;
                self.visit(loc_y)?;
                self.visit(width)?;
                self.visit(height)?;
                self.visit(colour)?;
                Ok(())
            }
            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                self.visit(loc_x)?;
                self.visit(loc_y)?;
                self.visit(colour)?;
                Ok(())
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                self.visit(condition)?;
                self.visit(if_true)?;
                if let Some(if_false) = if_false {
                    self.visit(if_false)?;
                }
                Ok(())
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.symbol_table.push(SymbolTable::new());
                self.scope_peek_limit = self.symbol_table.len() - 1;
                self.inside_function = true;

                if let Some(initializer) = initializer {
                    self.visit(initializer)?;
                }

                self.visit(condition)?;

                if let Some(increment) = increment {
                    self.visit(increment)?;
                }

                self.visit(body)?;
                self.scope_peek_limit = 0;
                self.inside_function = false;
                self.symbol_table.pop();
                Ok(())
            }
            AstNode::While { condition, body } => {
                self.visit(condition)?;
                self.visit(body)?;
                Ok(())
            }
            AstNode::Print { expression } => {
                self.visit(expression)?;
                Ok(())
            }
            AstNode::PadClear { expr } => {
                self.visit(expr)?;
                Ok(())
            }
            AstNode::EndOfFile => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexing::Lexer,
        parsing::Parser,
        semantics::utils::{SymbolType, Type},
        utils::SimpleBuffer,
    };

    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;
    use std::path::Path;

    fn run_scope_checker(input: &str) -> Result<()> {
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input, Path::new(""), None);

        let tokens = lexer.lex().unwrap();

        let mut parser = Parser::new(&tokens, Path::new(""));
        let ast = parser.parse()?;

        let mut scope_checker = TypeChecker::new();
        scope_checker.visit(ast)?;

        Ok(())
    }

    #[rstest]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();

        symbol_table.add_symbol("x", &SymbolType::Variable(Type::Int));
        symbol_table.add_symbol("y", &SymbolType::Variable(Type::Float));
        symbol_table.add_symbol("z", &SymbolType::Variable(Type::Bool));

        assert_matches!(
            symbol_table.find_symbol("x").unwrap().symbol_type,
            SymbolType::Variable(Type::Int)
        );

        assert_matches!(
            symbol_table.find_symbol("y").unwrap().symbol_type,
            SymbolType::Variable(Type::Float)
        );

        assert_matches!(
            symbol_table.find_symbol("z").unwrap().symbol_type,
            SymbolType::Variable(Type::Bool)
        );
    }

    #[rstest]
    fn test_scope_checker() {
        let input = r#"
            let x: int = 5;
            let y: float = 3.14;
            let z: bool = true;
            let f: colour = #ff0000;

            fun foo(x: int, y: float) -> int {
                let z: bool = false;
                let f: colour = #00ff00;
                return x;
            }

            let a: int = foo(5, 3.14);
            let b: float = x + y;
        "#;

        assert!(run_scope_checker(input).is_ok());
    }
}
