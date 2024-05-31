use crate::semantics::utils::{Signature, Symbol, SymbolTable, SymbolType};
use crate::{
    generation::instructions::{Instruction, Program},
    semantics::utils::Type,
};

use crate::core::{AstNode, Token, Visitor};
use crate::{core::TokenKind, semantics::utils::MemoryLocation};

#[derive(Debug)]
pub struct PArIRWriter {
    /// Stack of symbol tables, each representing a scope
    symbol_table: Vec<SymbolTable>,
    /// The ParIR program container
    program: Program,
    /// Pointer to the current instruction
    instr_ptr: usize,
    /// The current stack level
    stack_level: usize,
    /// The current frame index
    frame_index: usize,
}

impl PArIRWriter {
    pub fn new() -> Self {
        PArIRWriter {
            symbol_table: Vec::new(),
            program: Program {
                main: Vec::new(),
                functions: Vec::new(),
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
        self.program.main.push(instruction);
        self.instr_ptr += 1;
        self.instr_ptr - 1
    }

    fn get_scope_var_count(&self) -> usize {
        self.current_scope()
            .symbols
            .iter()
            .map(|s| match s.symbol_type {
                SymbolType::Array(_, size) => size,
                SymbolType::Variable(_) => 1,
                _ => 0,
            })
            .sum()
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

    fn add_symbol(
        &mut self,
        symbol: &Token,
        symbol_type: &SymbolType,
        mem_loc: Option<MemoryLocation>,
    ) {
        self.mut_current_scope()
            .add_symbol(&symbol.span.lexeme, symbol_type, mem_loc);
    }

    fn check_scope(&self, symbol: &Token) -> bool {
        self.current_scope()
            .find_symbol(&symbol.span.lexeme)
            .is_some()
    }

    fn get_memory_location(&self, symbol: &Token) -> Option<MemoryLocation> {
        self.find_symbol(symbol).and_then(|s| {
            let relative_mem_loc = s.memory_location;
            relative_mem_loc.map(|mem_loc| MemoryLocation {
                stack_level: self.stack_level - mem_loc.stack_level,
                frame_index: mem_loc.frame_index,
            })
        })
    }

    fn push_scope(&mut self) {
        self.symbol_table.push(SymbolTable::new());
    }

    fn pop_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn visit_unscoped_block(&mut self, block_node: &AstNode) -> usize {
        match block_node {
            AstNode::Block { statements } => {
                for statement in statements {
                    self.visit(statement);
                }
                self.instr_ptr
            }
            _ => unreachable!(),
        }
    }
}

impl Visitor<usize> for PArIRWriter {
    fn visit(&mut self, node: &AstNode) -> usize {
        match node {
            AstNode::Program { statements } => {
                self.push_scope();
                self.add_instruction(Instruction::FunctionLabel("main".to_string()));

                let var_count_push = self.add_instruction(Instruction::PushIntValue(0));
                self.add_instruction(Instruction::NewFrame);

                for statement in statements {
                    self.visit(statement);
                }

                self.program.main[var_count_push] =
                    Instruction::PushIntValue(self.get_scope_var_count());

                self.add_instruction(Instruction::PopFrame);
                self.add_instruction(Instruction::Halt);
                self.pop_scope();
            }

            AstNode::ArrayAccess { identifier, index } => {
                self.visit(index);

                let mem_loc = self.get_memory_location(identifier);

                if let Some(mem_loc) = mem_loc {
                    self.add_instruction(Instruction::PushOffsetFromOpS(mem_loc));
                }
            }

            AstNode::VarDecArray {
                identifier,
                element_type,
                size,
                elements,
            } => {
                for element in elements.iter().rev() {
                    self.visit(element);
                }

                let element_type = self
                    .current_scope()
                    .token_to_type(&element_type.span.lexeme);

                if !self.check_scope(identifier) {
                    self.add_symbol(
                        identifier,
                        &SymbolType::Array(element_type, *size),
                        Some(MemoryLocation {
                            stack_level: self.stack_level,
                            frame_index: self.frame_index,
                        }),
                    );
                }
                self.add_instruction(Instruction::PushIntValue(*size));

                self.add_instruction(Instruction::PushIntValue(self.frame_index));
                self.add_instruction(Instruction::PushIntValue(0));
                self.frame_index += size;

                self.add_instruction(Instruction::StoreArray);
            }
            AstNode::Block { statements } => {
                self.push_scope();
                let var_dec_count = self.add_instruction(Instruction::PushIntValue(0));
                self.add_instruction(Instruction::NewFrame);
                self.stack_level += 1;
                self.frame_index = 0;
                for statement in statements {
                    // if the statement is a return statement, we don't need to
                    // check the rest of the block
                    if let AstNode::Return { expression } = statement {
                        self.visit(expression);
                        self.program.main[var_dec_count] =
                            Instruction::PushIntValue(self.get_scope_var_count());
                        self.add_instruction(Instruction::Return);
                        self.add_instruction(Instruction::PopFrame);
                        self.stack_level -= 1;
                        self.pop_scope();
                        return self.instr_ptr;
                    } else {
                        self.visit(statement);
                    }
                }

                self.program.main[var_dec_count] =
                    Instruction::PushIntValue(self.get_scope_var_count());

                self.add_instruction(Instruction::PopFrame);
                self.stack_level -= 1;
                self.pop_scope();
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                self.add_symbol(
                    identifier,
                    &SymbolType::Function(Signature::new(return_type.clone())),
                    None,
                );

                self.push_scope();
                self.stack_level += 1;
                self.frame_index = 0;

                // Add the parameter symbols to the symbol table in this scope
                for param in params {
                    self.visit(param);
                }

                // all the parameters are added to the symbol table
                // now we add them to the function signature
                let mut signature = Signature::new(return_type.clone());

                for param in self.current_scope().all_symbols() {
                    if let SymbolType::Variable(t) = param.symbol_type.clone() {
                        signature.parameters.push((t, param.lexeme.clone()));
                    }
                }

                let start = self.instr_ptr;
                let end = self.visit_unscoped_block(block);
                let var_count = self.get_scope_var_count();

                self.program.functions.extend([
                    Instruction::FunctionLabel(identifier.span.lexeme.clone()),
                    Instruction::PushIntValue(var_count),
                    Instruction::Alloc,
                ]);

                self.program
                    .functions
                    .extend(self.program.main.drain(start..end));

                self.pop_scope();
                self.stack_level = 0;
                self.instr_ptr -= end - start;
                self.frame_index = 0;
            }

            AstNode::FunctionCall { identifier, args } => {
                let mut len = 0;

                for arg in args.iter().rev() {
                    if let AstNode::Expression { expr, .. } = arg {
                        if let AstNode::Identifier { token } = expr.as_ref() {
                            let symbol = self.find_symbol(token).unwrap();
                            if let SymbolType::Array(_type, s) = symbol.symbol_type.clone() {
                                len += s;
                            } else {
                                len += 1;
                            }
                        } else {
                            len += 1;
                        }
                    }

                    self.visit(arg);
                }

                self.add_instruction(Instruction::PushIntValue(len));
                self.add_instruction(Instruction::PushFunction(identifier.clone()));
                self.add_instruction(Instruction::Call);
            }

            AstNode::Identifier { token } => {
                let symbol = self.find_symbol(token).unwrap();

                match symbol.symbol_type {
                    SymbolType::Array(_, s) => {
                        if let Some(mem_loc) = self.get_memory_location(token) {
                            self.add_instruction(Instruction::PushIntValue(s));
                            self.add_instruction(Instruction::PushArray(mem_loc));
                            // workaround
                            self.add_instruction(Instruction::PushIntValue(s)); // push s
                            self.add_instruction(Instruction::NewFrame); // oframe
                            self.add_instruction(Instruction::PushIntValue(s)); // push s
                            self.add_instruction(Instruction::PushIntValue(0)); // push 0
                            self.add_instruction(Instruction::PushIntValue(0)); // push 0
                            self.add_instruction(Instruction::StoreArray);
                            self.add_instruction(Instruction::PushIntValue(s));
                            self.add_instruction(Instruction::PushArray(MemoryLocation {
                                stack_level: 0,
                                frame_index: 0,
                            }));
                            self.add_instruction(Instruction::PopFrame);
                            // end workaround
                        }
                    }
                    _ => {
                        if let Some(mem_loc) = self.get_memory_location(token) {
                            return self.add_instruction(Instruction::PushFromStack(mem_loc));
                        }
                    }
                }
            }

            AstNode::VarDec {
                identifier,
                var_type: r#type,
                expression,
            } => {
                self.visit(expression);

                if !self.check_scope(identifier) {
                    self.add_symbol(
                        identifier,
                        &SymbolType::Variable(
                            self.current_scope().token_to_type(&r#type.span.lexeme),
                        ),
                        Some(MemoryLocation {
                            stack_level: self.stack_level,
                            frame_index: self.frame_index,
                        }),
                    );
                }

                self.add_instruction(Instruction::PushIntValue(self.frame_index));
                self.add_instruction(Instruction::PushIntValue(0));
                self.frame_index += 1;
                self.add_instruction(Instruction::Store);
            }

            AstNode::FormalParam {
                identifier,
                param_type,
                length,
            } => match length {
                None => {
                    self.add_symbol(
                        identifier,
                        &SymbolType::Variable(
                            self.current_scope().token_to_type(&param_type.span.lexeme),
                        ),
                        Some(MemoryLocation {
                            stack_level: self.stack_level,
                            frame_index: self.frame_index,
                        }),
                    );
                    self.frame_index += 1;
                }
                Some(index) => {
                    let size: usize = index.span.lexeme.parse().unwrap();

                    self.add_symbol(
                        identifier,
                        &SymbolType::Array(
                            self.current_scope().token_to_type(&param_type.span.lexeme),
                            size,
                        ),
                        Some(MemoryLocation {
                            stack_level: self.stack_level,
                            frame_index: self.frame_index,
                        }),
                    );

                    self.frame_index += size;
                }
            },

            AstNode::Expression {
                casted_type: _,
                expr,
            } => {
                self.visit(expr);
            }

            AstNode::SubExpression { bin_op } => {
                self.visit(bin_op);
            }

            AstNode::Assignment {
                identifier,
                index,
                expression,
            } => {
                self.visit(expression);
                let mem_loc = self.get_memory_location(identifier);

                if let Some(mem_loc) = mem_loc {
                    if let Some(index) = index {
                        self.visit(index);
                    }

                    self.add_instruction(Instruction::PushIntValue(mem_loc.frame_index));
                    if index.is_some() {
                        self.add_instruction(Instruction::Add);
                    }
                    self.add_instruction(Instruction::PushIntValue(mem_loc.stack_level));
                    self.add_instruction(Instruction::Store);
                }
            }

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
                    TokenKind::Mod => Instruction::Mod,
                    TokenKind::Divide => Instruction::Div,
                    TokenKind::EqEq => Instruction::Equal,
                    TokenKind::LessThan => Instruction::LessThan,
                    TokenKind::LessThanEqual => Instruction::LessThanOrEqual,
                    TokenKind::GreaterThan => Instruction::GreaterThan,
                    TokenKind::GreaterThanEqual => Instruction::GreaterThanOrEqual,
                    TokenKind::And => Instruction::And,
                    TokenKind::Or => Instruction::Or,
                    _ => Instruction::NoOperation,
                });
            }

            AstNode::UnaryOp { operator, expr } => {
                self.visit(expr);

                match operator.kind {
                    TokenKind::Minus => self.add_instruction(Instruction::Sub),
                    TokenKind::Not => self.add_instruction(Instruction::Not),
                    _ => unreachable!(),
                };
            }

            AstNode::PadWidth => {
                self.add_instruction(Instruction::Width);
            }

            AstNode::PadRandI { upper_bound } => {
                self.visit(upper_bound);

                self.add_instruction(Instruction::RandInt);
            }

            AstNode::PadHeight => {
                self.add_instruction(Instruction::Height);
            }

            AstNode::PadRead { x, y } => {
                self.visit(y);
                self.visit(x);

                self.add_instruction(Instruction::Read);
            }

            AstNode::IntLiteral(l) => {
                self.add_instruction(Instruction::PushIntValue(l.span.lexeme.parse().unwrap()));
            }

            AstNode::FloatLiteral(l) => {
                self.add_instruction(Instruction::PushFloatValue(l.span.lexeme.parse().unwrap()));
            }

            AstNode::BoolLiteral(l) => {
                match l.span.lexeme.as_str() {
                    "true" => self.add_instruction(Instruction::PushIntValue(1)),
                    "false" => self.add_instruction(Instruction::PushIntValue(0)),
                    _ => unreachable!(),
                };
            }

            AstNode::ColourLiteral(l) => {
                let colour = u32::from_str_radix(&l.span.lexeme[1..], 16).unwrap();

                self.add_instruction(Instruction::PushIntValue(colour as usize));
            }

            AstNode::Delay { expression } => {
                self.visit(expression);
                self.add_instruction(Instruction::Delay);
            }

            AstNode::Return { expression } => {
                self.visit(expression);

                self.add_instruction(Instruction::Return);
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

                self.add_instruction(Instruction::WriteBox);
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                self.visit(colour);
                self.visit(loc_y);
                self.visit(loc_x);

                self.add_instruction(Instruction::Write);
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                self.visit(condition);

                let jump_to_true = self.add_instruction(Instruction::PushOffsetFromPC(0));
                self.add_instruction(Instruction::JumpIfNotZero);

                if let Some(if_false) = if_false {
                    self.visit_unscoped_block(if_false);
                }

                let jump_to_end = self.add_instruction(Instruction::PushOffsetFromPC(
                    self.instr_ptr as i32 - jump_to_true as i32,
                ));

                self.add_instruction(Instruction::Jump);

                self.program.main[jump_to_true] =
                    Instruction::PushOffsetFromPC(self.instr_ptr as i32 - jump_to_true as i32);

                self.visit_unscoped_block(if_true);

                self.program.main[jump_to_end] =
                    Instruction::PushOffsetFromPC(self.instr_ptr as i32 - jump_to_end as i32);
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();
                let push_var_count_placeholder = self.add_instruction(Instruction::PushIntValue(0));
                self.add_instruction(Instruction::NewFrame);
                self.stack_level += 1;
                self.frame_index = 0;

                if let Some(initializer) = initializer {
                    self.visit(initializer);
                }

                let before_condition = self.instr_ptr;
                self.visit(condition);
                self.add_instruction(Instruction::Not);

                let jump_to_end_placeholder = self.add_instruction(Instruction::PushIntValue(0));
                self.add_instruction(Instruction::JumpIfNotZero);
                self.visit_unscoped_block(body);

                if let Some(increment) = increment {
                    self.visit(increment);
                }

                self.add_instruction(Instruction::PushOffsetFromPC(
                    before_condition as i32 - self.instr_ptr as i32,
                ));
                self.add_instruction(Instruction::Jump);

                self.program.main[push_var_count_placeholder] =
                    Instruction::PushIntValue(self.get_scope_var_count());

                let pop = self.add_instruction(Instruction::PopFrame);
                self.program.main[jump_to_end_placeholder] =
                    Instruction::PushOffsetFromPC(pop as i32 - jump_to_end_placeholder as i32);
                self.pop_scope();
                self.stack_level -= 1;
            }

            AstNode::While { condition, body } => {
                self.push_scope();
                self.stack_level += 1;
                self.frame_index = 0;

                let var_count_push = self.add_instruction(Instruction::PushIntValue(0));
                self.add_instruction(Instruction::NewFrame);

                let before_condition = self.instr_ptr;
                self.visit(condition);
                self.add_instruction(Instruction::Not);
                let jump_to_end = self.add_instruction(Instruction::PushIntValue(0));

                self.add_instruction(Instruction::JumpIfNotZero);
                self.visit_unscoped_block(body);
                self.add_instruction(Instruction::PushOffsetFromPC(
                    before_condition as i32 - self.instr_ptr as i32,
                ));
                self.add_instruction(Instruction::Jump);

                self.program.main[var_count_push] =
                    Instruction::PushIntValue(self.get_scope_var_count());

                self.stack_level -= 1;
                let pop = self.add_instruction(Instruction::PopFrame);
                self.program.main[jump_to_end] =
                    Instruction::PushOffsetFromPC(pop as i32 - jump_to_end as i32);

                self.pop_scope();
            }

            AstNode::Print { expression } => {
                self.visit(expression);

                if let AstNode::Expression { expr, .. } = expression.as_ref() {
                    match expr.as_ref() {
                        AstNode::FunctionCall { identifier, .. } => {
                            let return_type =
                                self.find_symbol(identifier).unwrap().symbol_type.clone();

                            match return_type {
                                SymbolType::Function(signature) => match signature.return_type {
                                    Type::Array(_, s) => {
                                        self.add_instruction(Instruction::PushIntValue(s));
                                        self.add_instruction(Instruction::PrintArray);
                                    }
                                    _ => {
                                        self.add_instruction(Instruction::Print);
                                    }
                                },
                                _ => {
                                    self.add_instruction(Instruction::Print);
                                }
                            }
                        }

                        AstNode::Identifier { token } => {
                            let symbol = self.find_symbol(token).unwrap();

                            match symbol.symbol_type {
                                SymbolType::Array(_, s) => {
                                    self.add_instruction(Instruction::PushIntValue(s));
                                    self.add_instruction(Instruction::PrintArray);
                                }
                                _ => {
                                    self.add_instruction(Instruction::Print);
                                }
                            }
                        }

                        _ => {
                            self.add_instruction(Instruction::Print);
                        }
                    }
                }
            }

            AstNode::PadClear { expr } => {
                self.visit(expr);
                self.add_instruction(Instruction::Clear);
            }

            AstNode::EndOfFile => {}
        }
        self.instr_ptr
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
