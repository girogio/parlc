use crate::generation::instructions::{Instruction, Program};
use crate::semantics::utils::{Signature, Symbol, SymbolTable, SymbolType, Type};
use crate::utils::errors::SemanticError;
use crate::{
    core::Token,
    parsing::ast::{AstNode, Visitor},
};
use crate::{core::TokenKind, semantics::utils::MemLoc};

#[derive(Debug)]
pub struct PArIRWriter {
    /// Stack of symbol tables, each representing a scope
    symbol_table: Vec<SymbolTable>,
    /// Flag to denote that the current scope lies within a function
    inside_function: bool,
    /// If this is 0, we can check for the existence of the symbol in any
    /// scope, up to the global scope.
    scope_peek_limit: usize,
    /// String containing the program's contents
    program: Program,
    /// Pointer to the current instruction
    instr_ptr: usize,
    /// The current stack level
    stack_level: usize,
    /// The current stack offset
    frame_index: usize,
}

impl PArIRWriter {
    pub fn new() -> Self {
        PArIRWriter {
            symbol_table: Vec::new(),
            inside_function: false,
            scope_peek_limit: 0,
            program: Program {
                instructions: Vec::new(),
            },
            instr_ptr: 0,
            stack_level: 0,
            frame_index: 0,
        }
    }

    pub fn get_program(&mut self, ast: &AstNode) -> String {
        self.visit(ast);
        format!("{}", self.program)
    }

    fn add_instruction(&mut self, instruction: Instruction) -> usize {
        self.program.instructions.push(instruction);
        self.instr_ptr += 1;
        self.instr_ptr - 1
    }

    fn get_scope_var_count(&self) -> usize {
        self.current_scope()
            .symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Variable(_)))
            .count()
    }

    fn find_symbol(&self, symbol: &Token) -> Option<&Symbol> {
        self.symbol_table
            .iter()
            .rev()
            .find_map(|table| table.find_symbol(&symbol.span.lexeme))
    }

    fn mut_find_symbol(&mut self, symbol: &Token) -> Option<&mut Symbol> {
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

    fn add_symbol(&mut self, symbol: &Token, symbol_type: &SymbolType, mem_loc: Option<MemLoc>) {
        self.mut_current_scope()
            .add_symbol(&symbol.span.lexeme, symbol_type, mem_loc);
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.current_scope()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn get_memory_location(&self, symbol: &Token) -> Option<MemLoc> {
        self.find_symbol(symbol)
            .and_then(|s| s.memory_location.clone())
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

impl Visitor<usize> for PArIRWriter {
    fn visit(&mut self, node: &AstNode) -> usize {
        match node {
            AstNode::Program { statements } => {
                self.push_scope();
                self.add_instruction(Instruction::FunctionLabel("main".to_string()));
                let ir = self.instr_ptr;
                self.add_instruction(Instruction::PushValue(0));
                self.add_instruction(Instruction::NewFrame);

                for statement in statements {
                    self.visit(statement);
                }

                self.program.instructions[ir] =
                    Instruction::PushValue(self.get_scope_var_count() as u32);
                self.add_instruction(Instruction::PopFrame);
                self.add_instruction(Instruction::Halt);

                self.pop_scope();
                self.instr_ptr
            }

            AstNode::Block { statements } => {
                if !self.inside_function {
                    self.push_scope();
                }
                for statement in statements {
                    // if the statement is a return statement, we don't need to
                    // check the rest of the block
                    let ir = self.instr_ptr;
                    self.add_instruction(Instruction::PushValue(0));
                    self.add_instruction(Instruction::NewFrame);
                    if let AstNode::Return { expression } = statement {
                        let tmp = self.visit(expression);
                        self.pop_scope();
                        return tmp;
                    } else {
                        self.visit(statement);
                    }

                    self.program.instructions[ir] =
                        Instruction::PushValue(self.get_scope_var_count() as u32);
                }

                self.pop_scope();

                self.add_instruction(Instruction::PopFrame)
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                self.push_scope();
                self.scope_peek_limit = self.symbol_table.len() - 1;

                // Add the parameter symbols to the symbol table in this scope
                for param in params {
                    self.visit(param);
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
                        None,
                    );

                self.inside_function = true;
                let block_end = self.visit(block);

                self.pop_scope();
                self.inside_function = false;
                self.scope_peek_limit = 0;
                block_end
            }

            AstNode::Identifier { token } => {
                if let Some(mem_loc) = self.get_memory_location(token) {
                    return self.add_instruction(Instruction::PushFromStack(mem_loc));
                }

                self.instr_ptr
            }

            AstNode::VarDec {
                identifier,
                r#type: var_type,
                expression,
            } => {
                self.visit(expression);

                if !self.check_scope(identifier) {
                    self.add_symbol(
                        identifier,
                        &SymbolType::Variable(
                            self.current_scope().token_to_type(&var_type.span.lexeme),
                        ),
                        Some(MemLoc {
                            stack_level: self.stack_level,
                            frame_index: self.frame_index,
                        }),
                    );
                }

                self.add_instruction(Instruction::PushValue(self.frame_index as u32));
                self.add_instruction(Instruction::PushValue(self.stack_level as u32));
                self.frame_index += 1;
                self.add_instruction(Instruction::Store)
            }

            // AstNode::FunctionCall { identifier, args } => {
            //     let signature = self.get_signature(identifier);

            //     let arg_types = args
            //         .iter()
            //         .map(|arg| self.visit(arg))
            //         .collect::<Vec<usize>>();

            //     for (idx, b) in args.iter().rev().enumerate() {
            //         let arg_type = self.visit(b);
            //     }

            //     self.get_symbol_type(identifier)
            // }
            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                self.add_symbol(
                    identifier,
                    &SymbolType::Variable(
                        self.current_scope().token_to_type(&param_type.span.lexeme),
                    ),
                    None,
                );

                self.instr_ptr
            }

            AstNode::Expression {
                casted_type: _,
                expr,
            } => self.visit(expr),

            AstNode::SubExpression { bin_op } => self.visit(bin_op),

            AstNode::Assignment {
                identifier: _,
                expression,
            } => self.visit(expression),

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                self.visit(right);
                self.visit(left);

                self.add_instruction(match operator.kind {
                    TokenKind::Plus => Instruction::Add,
                    TokenKind::Minus => Instruction::Sub,
                    TokenKind::Multiply => Instruction::Mul,
                    TokenKind::Divide => Instruction::Div,
                    TokenKind::EqEq => Instruction::Equal,
                    TokenKind::LessThan => Instruction::LessThan,
                    TokenKind::LessThanEqual => Instruction::LessThanOrEqual,
                    TokenKind::GreaterThan => Instruction::GreaterThan,
                    TokenKind::GreaterThanEqual => Instruction::GreaterThanOrEqual,
                    TokenKind::And => Instruction::And,
                    TokenKind::Or => Instruction::Or,
                    _ => Instruction::NoOperation,
                })
            }

            AstNode::UnaryOp { operator, expr } => {
                self.visit(expr);

                match operator.kind {
                    TokenKind::Minus => self.add_instruction(Instruction::Sub),
                    TokenKind::Not => self.add_instruction(Instruction::Not),
                    _ => unreachable!(),
                }
            }

            AstNode::PadWidth => self.add_instruction(Instruction::Width),

            AstNode::PadRandI { upper_bound } => {
                self.visit(upper_bound);

                self.add_instruction(Instruction::RandInt)
            }

            AstNode::PadHeight => self.add_instruction(Instruction::Height),

            AstNode::PadRead { x, y } => {
                self.visit(y);
                self.visit(x);

                self.add_instruction(Instruction::Read)
            }

            AstNode::IntLiteral(l) => {
                self.add_instruction(Instruction::PushValue(l.span.lexeme.parse().unwrap()))
            }

            AstNode::FloatLiteral(l) => {
                //TODO: Fix into string
                self.add_instruction(Instruction::PushValue(l.span.lexeme.parse().unwrap()))
            }

            AstNode::BoolLiteral(l) => match l.span.lexeme.as_str() {
                "true" => self.add_instruction(Instruction::PushValue(1)),
                "false" => self.add_instruction(Instruction::PushValue(0)),
                _ => unreachable!(),
            },

            AstNode::ColourLiteral(l) => {
                let colour = u32::from_str_radix(&l.span.lexeme[1..], 16).unwrap();

                self.add_instruction(Instruction::PushValue(colour))
            }

            AstNode::ActualParams { params } => {
                for param in params {
                    self.visit(param);
                }
                self.instr_ptr
            }

            AstNode::Delay { expression } => {
                self.visit(expression);
                self.add_instruction(Instruction::Delay)
            }

            AstNode::Return { expression } => {
                self.visit(expression);

                self.add_instruction(Instruction::Return)
            }

            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => {
                self.visit(colour);
                self.visit(height);
                self.visit(width);
                self.visit(loc_y);
                self.visit(loc_x);

                self.add_instruction(Instruction::WriteBox)
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                self.visit(colour);
                self.visit(loc_y);
                self.visit(loc_x);

                self.add_instruction(Instruction::Write)
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                self.visit(condition);

                let jump_to_true = self.add_instruction(Instruction::PushValue(0));

                self.add_instruction(Instruction::JumpIfNotZero);

                if let Some(if_false) = if_false {
                    self.visit(if_false)
                } else {
                    self.add_instruction(Instruction::NoOperation)
                };

                let jump_to_end = self.add_instruction(Instruction::PushValue(0));
                let end_if = self.add_instruction(Instruction::Jump);

                self.program.instructions[jump_to_true] = Instruction::PushValue(end_if as u32 + 1);

                let after_if_true = self.visit(if_true);

                self.program.instructions[jump_to_end] =
                    Instruction::PushValue(after_if_true as u32);

                self.instr_ptr
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();
                self.scope_peek_limit = 0;
                self.inside_function = true;
                if let Some(initializer) = initializer {
                    self.visit(initializer);
                }

                self.visit(condition);

                if let Some(increment) = increment {
                    self.visit(increment);
                }

                let body_type = self.visit(body);
                self.inside_function = false;

                body_type
            }

            // AstNode::While { condition, body } => {
            //     let condition_type = self.visit(condition);
            //     if condition_type != Type::Bool {
            //         self.results.add_error(SemanticError::TypeMismatch(
            //             "while".to_string(),
            //             condition_type,
            //             Type::Bool,
            //         ));
            //     }

            //     self.visit(body)
            // }
            AstNode::Print { expression } => {
                self.visit(expression);
                self.add_instruction(Instruction::Print)
            }

            AstNode::PadClear { expr } => {
                self.visit(expr);
                self.add_instruction(Instruction::Clear)
            }

            AstNode::FunctionCall { identifier, args } => todo!(),
            AstNode::While { condition, body } => todo!(),
            AstNode::EndOfFile => todo!(),
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

        let mut scope_checker = PArIRWriter::new();
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
