use crate::core::TokenKind;
use crate::semantics::utils::{Signature, Symbol, SymbolTable, SymbolType, Type};
use crate::utils::errors::SemanticError;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};

#[derive(Debug)]
pub struct SemanticResult {
    pub errors: Vec<SemanticError>,
    pub warnings: Vec<SemanticError>,
}

impl SemanticResult {
    fn new() -> Self {
        SemanticResult {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn add_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    fn add_warning(&mut self, warning: SemanticError) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

#[derive(Debug)]
pub struct SemanticAnalyser {
    /// Stack of symbol tables, each representing a scope
    symbol_table: Vec<SymbolTable>,
    /// Flag to denote that the current scope lies within a function
    inside_function: bool,
    /// If this is 0, we can check for the existence of the symbol in any
    /// scope, up to the global scope.
    scope_peek_limit: usize,
    /// The results of the semantic analysis
    results: SemanticResult,
}

impl SemanticAnalyser {
    pub fn new() -> Self {
        SemanticAnalyser {
            symbol_table: Vec::new(),
            inside_function: false,
            scope_peek_limit: 0,
            results: SemanticResult::new(),
        }
    }

    pub fn analyze(&mut self, ast: &AstNode) -> &SemanticResult {
        self.visit(ast);
        &self.results
    }

    fn find_symbol(&self, symbol: &Token) -> Option<&Symbol> {
        self.symbol_table
            .iter()
            .rev()
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
    }

    fn find_symbol_mut(&mut self, symbol: &Token) -> Option<&mut Symbol> {
        self.symbol_table
            .iter_mut()
            .rev()
            .find_map(|table| table.find_symbol_mut(&symbol.span.lexeme))
    }

    fn current_scope(&self) -> &SymbolTable {
        self.symbol_table.last().unwrap()
    }

    fn mut_current_scope(&mut self) -> &mut SymbolTable {
        self.symbol_table.last_mut().unwrap()
    }

    fn add_symbol(&mut self, symbol: &Token, symbol_type: &SymbolType) {
        self.mut_current_scope()
            .add_symbol(&symbol.span.lexeme, symbol_type, None);
    }

    fn get_symbol_type(&self, symbol: &Token) -> Type {
        self.find_symbol(symbol)
            .map(|s| match s.symbol_type.clone() {
                SymbolType::Variable(t) => t,
                SymbolType::Function(signature) => signature.return_type,
                SymbolType::Array(t, _) => t,
            })
            .unwrap_or(Type::Unknown)
    }

    fn get_signature(&self, symbol: &Token) -> Signature {
        self.find_symbol(symbol)
            .map(|s| match &s.symbol_type {
                SymbolType::Function(signature) => signature.clone(),
                _ => unreachable!(),
            })
            .unwrap_or(Signature::new(Type::Unknown))
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.current_scope()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn get_unary_op_type(&mut self, op: &Token, expr: &Type) -> Type {
        match (op.kind, expr) {
            (TokenKind::Minus, Type::Int) => Type::Int,
            (TokenKind::Minus, Type::Float) => Type::Float,
            (TokenKind::Not, Type::Bool) => Type::Bool,
            _ => {
                self.results
                    .add_error(SemanticError::InvalidOperation(op.clone()));
                Type::Unknown
            }
        }
    }

    fn get_bin_op_type(&mut self, op: &Token, left: &Type, right: &Type) -> Type {
        match (op.kind, left, right) {
            (TokenKind::Mod, Type::Int, Type::Int) => Type::Int,
            (
                TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide,
                Type::Int,
                Type::Int,
            ) => Type::Int,
            (
                TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide,
                Type::Float,
                Type::Int,
            ) => Type::Float,
            (
                TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide,
                Type::Int,
                Type::Float,
            ) => Type::Float,
            (
                TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide,
                Type::Float,
                Type::Float,
            ) => Type::Float,
            (
                TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide,
                Type::Colour,
                Type::Colour,
            ) => Type::Colour,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Int,
                Type::Int,
            ) => Type::Bool,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Int,
                Type::Float,
            ) => Type::Bool,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Float,
                Type::Int,
            ) => Type::Bool,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Float,
                Type::Float,
            ) => Type::Bool,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Bool,
                Type::Bool,
            ) => Type::Bool,
            (
                TokenKind::EqEq
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual,
                Type::Colour,
                Type::Colour,
            ) => Type::Bool,
            (TokenKind::And | TokenKind::Or, Type::Bool, Type::Bool) => Type::Bool,
            _ => {
                self.results
                    .add_error(SemanticError::InvalidOperation(op.clone()));

                Type::Unknown
            }
        }
    }

    fn assert_type(&mut self, token: &String, expected: &Type, found: &Type) -> Type {
        if expected != found {
            self.results.add_error(SemanticError::TypeMismatch(
                token.to_string(),
                found.clone(),
                expected.clone(),
            ));
        }

        found.clone()
    }

    fn check_up_to_scope(&self, symbol: &Token) -> bool {
        self.symbol_table
            .iter()
            .skip(self.scope_peek_limit)
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
            .is_some()
    }

    fn visit_unscoped_block(&mut self, block: &AstNode) -> Type {
        match block {
            AstNode::Block { statements } => {
                let last = Type::Void;
                for statement in statements {
                    let last = self.visit(statement);
                    if let AstNode::Return { .. } = statement {
                        return last;
                    }
                }
                last
            }
            _ => unreachable!(), // Unless called with a non-block node
        }
    }
    fn push_scope(&mut self) {
        self.symbol_table.push(SymbolTable::new());
    }

    fn pop_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn check_cast(&mut self, to: &Token, from: Type) -> Type {
        let to = self.current_scope().token_to_type(&to.span.lexeme);

        if from == to {
            return from;
        }

        match (from.clone(), to.clone()) {
            (Type::Int, Type::Float) => Type::Float,   // 5 -> 5.0
            (Type::Colour, Type::Int) => Type::Int,    // 0xRRGGBB -> 0xRR + 0xGG + 0xBB
            (Type::Bool, Type::Int) => Type::Int,      // false -> 0, true -> 1
            (Type::Int, Type::Colour) => Type::Colour, // 0xRR + 0xGG + 0xBB -> 0xRRGGBB
            (Type::Bool, Type::Float) => Type::Float,  // false -> 0.0, true -> 1.0
            _ => {
                self.results.add_error(SemanticError::InvalidCast(from, to));
                Type::Unknown
            }
        }
    }
}

impl Visitor<Type> for SemanticAnalyser {
    fn visit(&mut self, node: &AstNode) -> Type {
        match node {
            AstNode::Program { statements } => {
                self.push_scope();
                for statement in statements {
                    self.visit(statement);
                }
                self.pop_scope();

                Type::Void
            }
            AstNode::ArrayAccess { identifier, index } => {
                let identifier_type = self.get_symbol_type(identifier);
                let index_type = self.visit(index);

                if self.find_symbol(identifier).is_none() {
                    self.results
                        .add_error(SemanticError::UndefinedVariable(identifier.clone()));
                }

                if index_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "index".to_string(),
                        index_type,
                        Type::Int,
                    ));
                }

                identifier_type
            }

            AstNode::Block { statements } => {
                self.push_scope();
                let mut last = Type::Void;
                for statement in statements {
                    // if the statement is a return statement, we don't need to
                    // check the rest of the block
                    if let AstNode::Return { expression } = statement {
                        last = self.visit(expression);
                        break;
                    } else {
                        self.visit(statement);
                    }
                }
                self.pop_scope();
                last
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                // Check that function name isn't already defined
                if self.check_scope(identifier) {
                    self.results
                        .add_error(SemanticError::FunctionAlreadyDefined(identifier.clone()));
                } else {
                    // Add the function to the symbol table in the current scope
                    self.add_symbol(
                        identifier,
                        &SymbolType::Function(Signature::new(Type::Unknown)),
                    );
                }

                self.push_scope();

                // all the parameters are added to the symbol table
                // now we add them to the function signature
                let mut signature = Signature::new(return_type.clone());

                for param in params.iter().rev() {
                    let param_type = self.visit(param);

                    match param {
                        AstNode::FormalParam { identifier, .. } => {
                            signature
                                .parameters
                                .push((param_type, identifier.span.lexeme.clone()));
                        }
                        _ => unreachable!(),
                    }
                }

                self.find_symbol_mut(identifier).unwrap().symbol_type =
                    SymbolType::Function(signature.clone());

                self.inside_function = true;
                self.scope_peek_limit = self.symbol_table.len() - 1;
                let block_return_type = self.visit_unscoped_block(block);
                self.inside_function = false;

                if signature.return_type != block_return_type {
                    self.results
                        .add_error(SemanticError::FunctionReturnTypeMismatch(
                            identifier.clone(),
                            signature.return_type,
                            block_return_type,
                        ));
                }

                self.pop_scope();

                Type::Void
            }

            AstNode::Identifier { token } => {
                if self.inside_function {
                    if !self.check_up_to_scope(token) {
                        self.results
                            .add_error(SemanticError::VarUndefinedInFunc(token.clone()));
                    }
                } else if self.find_symbol(token).is_none() {
                    self.results
                        .add_error(SemanticError::UndefinedVariable(token.clone()));
                }

                self.find_symbol(token)
                    .map(|s| match s.symbol_type.clone() {
                        SymbolType::Variable(t) => t,
                        SymbolType::Function(signature) => signature.return_type,
                        SymbolType::Array(t, s) => Type::Array(Box::new(t), s),
                    })
                    .unwrap_or(Type::Unknown)
            }

            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                let expr_type = self.visit(expression);

                if self.check_scope(identifier) {
                    self.results
                        .add_error(SemanticError::VariableRedeclaration(identifier.clone()));
                } else {
                    // if the variable was not found in this scope, emit a shadowing warning
                    if self.find_symbol(identifier).is_some() {
                        self.results
                            .add_warning(SemanticError::VariableShadowing(identifier.clone()));
                    }

                    self.add_symbol(
                        identifier,
                        &SymbolType::Variable(
                            self.current_scope().token_to_type(&var_type.span.lexeme),
                        ),
                    );
                }

                self.assert_type(
                    &identifier.span.lexeme,
                    &self.current_scope().token_to_type(&var_type.span.lexeme),
                    &expr_type,
                );

                Type::Void
            }

            AstNode::VarDecArray {
                identifier,
                element_type,
                size,
                elements,
            } => {
                let element_type = self
                    .current_scope()
                    .token_to_type(&element_type.span.lexeme);

                if self.check_scope(identifier) {
                    self.results
                        .add_error(SemanticError::VariableRedeclaration(identifier.clone()));
                } else {
                    self.add_symbol(identifier, &SymbolType::Array(element_type.clone(), *size));
                }

                if elements.len() > *size {
                    self.results.add_error(SemanticError::ArrayOverflow(
                        identifier.clone(),
                        element_type.clone(),
                        *size,
                        elements.len(),
                    ));
                }

                for element in elements {
                    let current_element_type = self.visit(element);

                    if element_type != current_element_type {
                        self.results.add_error(SemanticError::TypeMismatch(
                            "element".to_string(),
                            element_type.clone(),
                            element_type.clone(),
                        ));
                    }
                }

                Type::Void
            }

            AstNode::FunctionCall { identifier, args } => {
                if self.find_symbol(identifier).is_none() {
                    self.results
                        .add_error(SemanticError::UndefinedFunction(identifier.clone()))
                }

                let signature = self.get_signature(identifier);

                if signature.return_type == Type::Unknown {
                    return Type::Unknown;
                }

                let arg_types = args
                    .iter()
                    .map(|arg| self.visit(arg))
                    .collect::<Vec<Type>>();

                if signature.parameters.is_empty() && !arg_types.is_empty() {
                    self.results.add_error(SemanticError::FunctionCallNoParams(
                        identifier.span.lexeme.clone(),
                        arg_types,
                    ));
                }

                // Make sure each argument is of the correct type
                for (idx, b) in args.iter().rev().enumerate() {
                    let arg_type = self.visit(b);

                    self.assert_type(
                        &signature.parameters[idx].1,
                        &signature.parameters[idx].0,
                        &arg_type,
                    );
                }

                self.get_symbol_type(identifier)
            }

            AstNode::FormalParam {
                identifier,
                param_type,
                index,
            } => match index {
                None => {
                    let param_type = self.current_scope().token_to_type(&param_type.span.lexeme);
                    self.add_symbol(identifier, &SymbolType::Variable(param_type.clone()));
                    param_type
                }
                Some(index) => {
                    let size: usize = index.span.lexeme.parse().unwrap();
                    self.add_symbol(
                        identifier,
                        &SymbolType::Array(
                            self.current_scope().token_to_type(&param_type.span.lexeme),
                            size,
                        ),
                    );
                    Type::Array(
                        Box::new(self.current_scope().token_to_type(&param_type.span.lexeme)),
                        size,
                    )
                }
            },

            AstNode::Expression { casted_type, expr } => {
                let expr_type = self.visit(expr);

                match casted_type {
                    Some(casted_type) => self.check_cast(casted_type, expr_type),
                    None => expr_type,
                }
            }

            AstNode::SubExpression { bin_op } => self.visit(bin_op),

            AstNode::Assignment {
                identifier,
                index,
                expression,
            } => {
                if self.inside_function {
                    if !self.check_up_to_scope(identifier) {
                        self.results
                            .add_error(SemanticError::VarUndefinedInFunc(identifier.clone()));
                    }
                } else if self.find_symbol(identifier).is_none() {
                    self.results
                        .add_error(SemanticError::UndefinedVariable(identifier.clone()));
                }

                let identifier_type = self.get_symbol_type(identifier);

                if let Some(index) = index {
                    let index_type = self.visit(index);

                    if index_type != Type::Int {
                        self.results.add_error(SemanticError::ArrayIndexNotInt(
                            identifier.clone(),
                            index_type,
                        ));
                    }
                }

                let expression_type = self.visit(expression);

                self.assert_type(&identifier.span.lexeme, &identifier_type, &expression_type)
                    .clone()
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                let left_type = self.visit(left);
                let right_type = self.visit(right);

                self.get_bin_op_type(operator, &left_type, &right_type)
            }

            AstNode::UnaryOp { operator, expr } => {
                let expr_type = self.visit(expr);

                self.get_unary_op_type(operator, &expr_type)
            }

            AstNode::PadWidth => Type::Int,

            AstNode::PadRandI { upper_bound } => {
                let upper_bound_type = self.visit(upper_bound);

                if upper_bound_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "upper_bound".to_string(),
                        upper_bound_type,
                        Type::Int,
                    ));
                }

                Type::Int
            }

            AstNode::PadHeight => Type::Int,

            AstNode::PadRead { x, y } => {
                let x_type = self.visit(x);
                let y_type = self.visit(y);

                if x_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "__read <x>, y".to_string(),
                        x_type,
                        Type::Int,
                    ));
                }

                if y_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "__read x, <y>".to_string(),
                        y_type,
                        Type::Int,
                    ));
                }

                Type::Int
            }

            AstNode::IntLiteral(_) => Type::Int,

            AstNode::FloatLiteral(_) => Type::Float,

            AstNode::BoolLiteral(_) => Type::Bool,

            AstNode::ColourLiteral(_) => Type::Colour,

            AstNode::ActualParams { params } => {
                for param in params {
                    self.visit(param);
                }

                Type::Void
            }
            AstNode::Delay { expression } => {
                let delay_ms_type = self.visit(expression);

                if delay_ms_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "delay".to_string(),
                        delay_ms_type,
                        Type::Int,
                    ));
                }

                Type::Void
            }

            AstNode::Return { expression } => self.visit(expression),

            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => {
                let loc_x_type = self.visit(loc_x);
                let loc_y_type = self.visit(loc_y);
                let width_type = self.visit(width);
                let height_type = self.visit(height);
                let colour_type = self.visit(colour);

                if loc_x_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "loc_x".to_string(),
                        loc_x_type,
                        Type::Int,
                    ));
                }

                if loc_y_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "loc_y".to_string(),
                        loc_y_type,
                        Type::Int,
                    ));
                }

                if width_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "width".to_string(),
                        width_type,
                        Type::Int,
                    ));
                }

                if height_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "height".to_string(),
                        height_type,
                        Type::Int,
                    ));
                }

                if colour_type != Type::Colour {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "colour".to_string(),
                        colour_type,
                        Type::Colour,
                    ));
                }

                Type::Void
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                let loc_x_type = self.visit(loc_x);
                let loc_y_type = self.visit(loc_y);
                let colour_type = self.visit(colour);

                if loc_x_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "loc_x".to_string(),
                        loc_x_type,
                        Type::Int,
                    ));
                }

                if loc_y_type != Type::Int {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "loc_y".to_string(),
                        loc_y_type,
                        Type::Int,
                    ));
                }

                if colour_type != Type::Colour {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "colour".to_string(),
                        colour_type,
                        Type::Colour,
                    ));
                }

                Type::Void
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                self.visit(condition);
                let true_branch_return_type = self.visit_unscoped_block(if_true);
                if let Some(if_false) = if_false {
                    let false_branch_return_type = self.visit_unscoped_block(if_false);

                    if true_branch_return_type != false_branch_return_type {
                        self.results.add_error(SemanticError::TypeMismatch(
                            "if".to_string(),
                            true_branch_return_type.clone(),
                            false_branch_return_type,
                        ));
                    }
                }

                true_branch_return_type
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();

                if let Some(initializer) = initializer {
                    self.visit(initializer);
                }

                let condition_type = self.visit(condition);

                if condition_type != Type::Bool {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "for condition".to_string(),
                        condition_type,
                        Type::Bool,
                    ));
                }

                if let Some(increment) = increment {
                    self.visit(increment);
                }

                let body_type = self.visit_unscoped_block(body);
                self.symbol_table.pop();

                body_type
            }

            AstNode::While { condition, body } => {
                self.push_scope();
                let condition_type = self.visit(condition);
                if condition_type != Type::Bool {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "while".to_string(),
                        condition_type,
                        Type::Bool,
                    ));
                }
                let body_return_type = self.visit_unscoped_block(body);
                self.pop_scope();

                body_return_type
            }

            AstNode::Print { expression } => {
                let print_expr_type = self.visit(expression);

                if print_expr_type == Type::Void || print_expr_type == Type::Unknown {
                    self.results.add_error(SemanticError::TypeMismatchUnion(
                        "__print <expr>".to_string(),
                        print_expr_type,
                        vec![Type::Int, Type::Float, Type::Bool, Type::Colour],
                    ));
                }

                Type::Void
            }

            AstNode::PadClear { expr } => {
                let clear_expr_type = self.visit(expr);

                if clear_expr_type != Type::Colour {
                    self.results.add_error(SemanticError::TypeMismatch(
                        "__clear <expr>".to_string(),
                        clear_expr_type,
                        Type::Colour,
                    ));
                }

                Type::Void
            }

            AstNode::EndOfFile => Type::Void,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexing::Lexer,
        parsing::Parser,
        semantics::utils::{SymbolType, Type},
        utils::{errors::Error, SimpleBuffer},
    };

    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;
    use std::path::Path;

    fn run_scope_checker(input: &str) -> Result<(), Error> {
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input, Path::new(""), None);

        let tokens = lexer.lex().unwrap();

        let mut parser = Parser::new(&tokens, Path::new(""));
        let ast = parser.parse()?;

        let mut scope_checker = SemanticAnalyser::new();
        scope_checker.visit(ast);

        Ok(())
    }

    #[rstest]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();

        symbol_table.add_symbol("x", &SymbolType::Variable(Type::Int), None);
        symbol_table.add_symbol("y", &SymbolType::Variable(Type::Float), None);
        symbol_table.add_symbol("z", &SymbolType::Variable(Type::Bool), None);

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

            fun foo(x: int, y: float) -> bool {
                let z: bool = false;
                let f: colour = #00ff00;
                return z;
            }

            let a: bool = foo(5, 3.14);
            let b: float = x + y;
        "#;

        assert!(run_scope_checker(input).is_ok());
    }
}
