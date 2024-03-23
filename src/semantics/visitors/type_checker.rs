use crate::core::TokenKind;
use crate::semantics::utils::{Signature, Symbol, SymbolTable, SymbolType, Type};
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

    fn add_symbol(&mut self, symbol: &Token, symbol_type: &SymbolType) {
        self.mut_current_scope()
            .add_symbol(&symbol.span.lexeme, symbol_type);
    }

    fn get_symbol_type(&self, symbol: &Token) -> Result<Type> {
        self.find_symbol(symbol)
            .map(|s| match &s.symbol_type {
                SymbolType::Variable(t) => *t,
                SymbolType::Function(signature) => signature.return_type,
            })
            .ok_or_else(|| SemanticError::UndefinedVariable(symbol.clone()).into())
    }

    fn get_signature(&self, symbol: &Token) -> Result<&Signature> {
        self.find_symbol(symbol)
            .map(|s| match &s.symbol_type {
                SymbolType::Function(signature) => signature,
                _ => panic!("Symbol is not a function"),
            })
            .ok_or_else(|| SemanticError::UndefinedFunction(symbol.clone()).into())
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.current_scope()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn get_unary_op_type(&self, op: &Token, expr: &Type) -> Result<Type> {
        match (op.kind, expr) {
            (TokenKind::Minus, Type::Int) => Ok(Type::Int),
            (TokenKind::Minus, Type::Float) => Ok(Type::Float),
            (TokenKind::Not, Type::Bool) => Ok(Type::Bool),
            _ => Err(SemanticError::InvalidOperation(op.clone()).into()),
        }
    }

    fn get_bin_op_type(&self, op: &Token, left: &Type, right: &Type) -> Result<Type> {
        match (op.kind, left, right) {
            (TokenKind::Plus, Type::Int, Type::Int) => Ok(Type::Int),
            (TokenKind::Plus, Type::Float, Type::Float) => Ok(Type::Float),
            (TokenKind::Plus, Type::Colour, Type::Colour) => Ok(Type::Colour),
            (TokenKind::Minus, Type::Int, Type::Int) => Ok(Type::Int),
            (TokenKind::Minus, Type::Float, Type::Float) => Ok(Type::Float),
            (TokenKind::Minus, Type::Colour, Type::Colour) => Ok(Type::Colour),
            (TokenKind::Multiply, Type::Int, Type::Int) => Ok(Type::Int),
            (TokenKind::Multiply, Type::Float, Type::Float) => Ok(Type::Float),
            (TokenKind::Multiply, Type::Colour, Type::Colour) => Ok(Type::Colour),
            (TokenKind::Divide, Type::Int, Type::Int) => Ok(Type::Int),
            (TokenKind::Divide, Type::Float, Type::Float) => Ok(Type::Float),
            (TokenKind::Divide, Type::Colour, Type::Colour) => Ok(Type::Colour),
            (TokenKind::EqEq, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::EqEq, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::EqEq, Type::Bool, Type::Bool) => Ok(Type::Bool),
            (TokenKind::EqEq, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::NotEqual, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::NotEqual, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::NotEqual, Type::Bool, Type::Bool) => Ok(Type::Bool),
            (TokenKind::NotEqual, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::LessThan, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::LessThan, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::LessThan, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::LessThanEqual, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::LessThanEqual, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::LessThanEqual, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::GreaterThan, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::GreaterThan, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::GreaterThan, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::GreaterThanEqual, Type::Int, Type::Int) => Ok(Type::Bool),
            (TokenKind::GreaterThanEqual, Type::Float, Type::Float) => Ok(Type::Bool),
            (TokenKind::GreaterThanEqual, Type::Colour, Type::Colour) => Ok(Type::Bool),
            (TokenKind::And, Type::Bool, Type::Bool) => Ok(Type::Bool),
            (TokenKind::Or, Type::Bool, Type::Bool) => Ok(Type::Bool),
            _ => Err(SemanticError::InvalidOperation(op.clone()).into()),
        }
    }

    fn assert_type(&self, token: &String, expected: &Type, found: &Type) -> Result<Type> {
        if expected != found {
            return Err(SemanticError::TypeMismatch(token.to_string(), *found, *expected).into());
        }

        Ok(*found)
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

impl Visitor<Type> for TypeChecker {
    fn visit(&mut self, node: &AstNode) -> Result<Type> {
        match node {
            AstNode::Program { statements } => {
                self.push_scope();
                for statement in statements {
                    self.visit(statement)?;
                }
                self.pop_scope();

                Ok(Type::Void)
            }

            AstNode::Block { statements } => {
                self.push_scope();
                for statement in statements {
                    // if the statement is a return statement, we don't need to
                    // check the rest of the block
                    if let AstNode::Return { .. } = statement {
                        let tmp = self.visit(statement);
                        self.pop_scope();
                        return tmp;
                    } else {
                        self.visit(statement)?;
                    }
                }
                self.pop_scope();

                Ok(Type::Void)
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
                        signature.parameters.push((t, param.lexeme.clone()));
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
                self.pop_scope();
                self.inside_function = false;
                self.scope_peek_limit = 0;

                Ok(Type::Void)
            }

            AstNode::Identifier { token } => {
                if self.inside_function {
                    if !self.check_up_to_scope(token) {
                        return Err(SemanticError::UndefinedVariable(token.clone()).into());
                    }
                } else if !self.check_scope(token) {
                    return Err(SemanticError::UndefinedVariable(token.clone()).into());
                }

                self.get_symbol_type(token)
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
                    );
                }

                Ok(Type::Void)
            }

            AstNode::FunctionCall { identifier, args } => {
                if self.find_symbol(identifier).is_none() {
                    return Err(SemanticError::UndefinedFunction(identifier.clone()).into());
                }

                let signature = self.get_signature(identifier)?.clone();

                args.iter()
                    .rev()
                    .enumerate()
                    .try_for_each(|(i, arg)| -> Result<()> {
                        let arg_type = self.visit(arg)?;
                        self.assert_type(
                            &signature.parameters[i].1,
                            &signature.parameters[i].0,
                            &arg_type,
                        )
                        .map(|_| ())
                    })?;

                self.get_symbol_type(identifier)
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
                );

                Ok(Type::Void)
            }

            AstNode::Expression {
                casted_type: _,
                expr: bin_op,
            } => self.visit(bin_op),

            AstNode::SubExpression { bin_op } => self.visit(bin_op),

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

                let identifier_type = self.get_symbol_type(identifier)?;
                let expression_type = self.visit(expression)?;

                self.assert_type(&identifier.span.lexeme, &identifier_type, &expression_type)
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                let left_type = self.visit(left)?;
                let right_type = self.visit(right)?;

                self.get_bin_op_type(operator, &left_type, &right_type)
            }

            AstNode::UnaryOp { operator, expr } => {
                let expr_type = self.visit(expr)?;

                self.get_unary_op_type(operator, &expr_type)
            }

            AstNode::PadWidth => Ok(Type::Int),

            AstNode::PadRandI { upper_bound } => {
                self.visit(upper_bound)?;
                Ok(Type::Int)
            }

            AstNode::PadHeight => Ok(Type::Int),

            AstNode::PadRead { first, second } => {
                self.visit(first)?;
                self.visit(second)?;

                Ok(Type::Int)
            }

            AstNode::IntLiteral(_) => Ok(Type::Int),

            AstNode::FloatLiteral(_) => Ok(Type::Float),

            AstNode::BoolLiteral(_) => Ok(Type::Bool),

            AstNode::ColourLiteral(_) => Ok(Type::Colour),

            AstNode::ActualParams { params } => {
                for param in params {
                    self.visit(param)?;
                }

                Ok(Type::Void)
            }
            AstNode::Delay { expression } => {
                self.visit(expression)?;

                Ok(Type::Void)
            }

            AstNode::Return { expression } => self.visit(expression),

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

                Ok(Type::Void)
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                self.visit(loc_x)?;
                self.visit(loc_y)?;
                self.visit(colour)?;

                Ok(Type::Void)
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

                Ok(Type::Void)
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

                Ok(Type::Void)
            }
            AstNode::While { condition, body } => {
                self.visit(condition)?;
                self.visit(body)?;
                Ok(Type::Void)
            }
            AstNode::Print { expression } => {
                self.visit(expression)?;
                Ok(Type::Void)
            }
            AstNode::PadClear { expr } => {
                self.visit(expr)?;

                Ok(Type::Void)
            }
            AstNode::EndOfFile => Ok(Type::Void),
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
    fn test_type_checker() {
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
