use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::parsing::ast::{AstNode, Visitor};
use crate::utils::Result;

pub struct Formatter {
    tab_level: usize,
    buff: BufWriter<File>,
}

impl Formatter {
    pub fn new(file: &Path) -> Self {
        let file = File::create(file).unwrap();

        Self {
            tab_level: 0,
            buff: BufWriter::new(file),
        }
    }
}

impl Visitor<Result<()>> for Formatter {
    fn visit(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program { statements } => {
                for statement in statements {
                    self.visit(statement)?;
                    if let AstNode::Assignment { .. } = statement {
                        writeln!(self.buff, ";")?;
                    }
                    writeln!(self.buff, "\n")?;
                }
                Ok(())
            }

            AstNode::ArrayAccess { identifier, index } => {
                write!(self.buff, "{}[", identifier.span.lexeme)?;
                self.visit(index)?;
                write!(self.buff, "]")?;
                Ok(())
            }

            AstNode::VarDecArray {
                identifier,
                element_type,
                size: _,
                elements,
            } => {
                write!(
                    self.buff,
                    "let {}: {}[] = [",
                    identifier.span.lexeme, element_type.span.lexeme
                )?;
                for (i, element) in elements.iter().enumerate() {
                    self.visit(element)?;
                    if i != elements.len() - 1 {
                        write!(self.buff, ", ")?;
                    }
                }
                write!(self.buff, "];")?;
                Ok(())
            }

            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                write!(self.buff, "let {}", identifier.span.lexeme)?;
                write!(self.buff, ": {}", var_type.span.lexeme)?;
                write!(self.buff, " = ")?;
                self.visit(expression)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::Delay { expression } => {
                write!(self.buff, "__delay ")?;
                self.visit(expression)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::Print { expression } => {
                write!(self.buff, "__print ")?;
                self.visit(expression)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                write!(self.buff, "if (")?;
                self.visit(condition)?;
                write!(self.buff, ")")?;
                let tmp_tab_level = self.tab_level;
                self.visit(if_true)?;
                self.tab_level = tmp_tab_level;
                if let Some(if_false) = if_false {
                    write!(self.buff, " else")?;
                    self.visit(if_false)?;
                }

                Ok(())
            }

            AstNode::Assignment {
                identifier,
                index,
                expression,
            } => {
                write!(self.buff, "{}", identifier.span.lexeme)?;
                if let Some(index) = index {
                    write!(self.buff, "[")?;
                    self.visit(index)?;
                    write!(self.buff, "]")?;
                }

                write!(self.buff, " = ")?;

                self.visit(expression)?;
                Ok(())
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                write!(self.buff, "for (")?;

                // This is to avoid printing newline after the variable declaration in a for loop
                if let Some(node) = initializer {
                    if let AstNode::VarDec {
                        identifier,
                        var_type,
                        expression,
                    } = node.as_ref()
                    {
                        write!(self.buff, "let {}", identifier.span.lexeme)?;
                        write!(self.buff, ": {} = ", var_type.span.lexeme)?;
                        self.visit(expression)?;
                    }
                }

                write!(self.buff, "; ")?;

                self.visit(condition)?;

                write!(self.buff, "; ")?;
                if let Some(increment) = increment.as_ref() {
                    self.visit(increment)?;
                }
                write!(self.buff, ")")?;
                self.visit(body)?;
                Ok(())
            }

            AstNode::Return { expression } => {
                write!(self.buff, "return ")?;
                self.visit(expression)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::Block { statements } => {
                writeln!(self.buff, " {{")?;
                self.tab_level += 1;
                for statement in statements {
                    write!(self.buff, "{}", "\t".repeat(self.tab_level))?;
                    self.visit(statement)?;
                    if let AstNode::Assignment { .. } = statement {
                        writeln!(self.buff, ";")?;
                    } else {
                        writeln!(self.buff)?;
                    }
                }
                self.tab_level -= 1;

                write!(self.buff, "{}}}", "\t".repeat(self.tab_level))?;
                Ok(())
            }

            AstNode::Expression {
                casted_type,
                expr: bin_op,
            } => {
                self.visit(bin_op)?;
                match casted_type {
                    Some(casted_type) => {
                        write!(self.buff, " as {}", casted_type.span.lexeme)?;
                    }
                    None => {}
                }
                Ok(())
            }

            AstNode::SubExpression { bin_op } => {
                write!(self.buff, "(")?;
                self.visit(bin_op)?;
                write!(self.buff, ")")?;
                Ok(())
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                write!(self.buff, "fun {}", identifier)?;
                write!(self.buff, "(")?;

                if params.is_empty() {
                    write!(self.buff, ") -> {}", return_type)?;
                    self.visit(block)?;
                    return Ok(());
                } else {
                    for param in params.iter().take(params.len() - 1) {
                        self.visit(param)?;
                        write!(self.buff, ", ")?;
                    }

                    if let Some(last) = params.last() {
                        self.visit(last)?;
                    }
                }

                write!(self.buff, ") -> {}", return_type)?;

                // if let Some(return_type_size) = return_type_size {
                //     write!(self.buff, "[{}]", return_type_size)?;
                // }

                self.visit(block)?;
                Ok(())
            }

            AstNode::While { condition, body } => {
                write!(self.buff, "while (")?;
                let prev_tab_level = self.tab_level;
                self.tab_level = 0;
                self.visit(condition)?;
                self.tab_level = prev_tab_level;
                write!(self.buff, ") ")?;
                self.visit(body)?;
                Ok(())
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                write!(self.buff, "__write ")?;
                self.visit(loc_x)?;
                write!(self.buff, ", ")?;
                self.visit(loc_y)?;
                write!(self.buff, ", ")?;
                self.visit(colour)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::PadClear { expr } => {
                write!(self.buff, "__clear ")?;

                self.visit(expr)?;

                write!(self.buff, ";")?;

                Ok(())
            }

            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => {
                write!(self.buff, "__write_box ")?;
                self.visit(loc_x)?;
                write!(self.buff, ", ")?;
                self.visit(loc_y)?;
                write!(self.buff, ", ")?;
                self.visit(width)?;
                write!(self.buff, ", ")?;
                self.visit(height)?;
                write!(self.buff, ", ")?;
                self.visit(colour)?;
                write!(self.buff, ";")?;
                Ok(())
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                self.visit(left)?;
                write!(self.buff, " {} ", operator.span.lexeme)?;
                self.visit(right)?;
                Ok(())
            }

            AstNode::UnaryOp { operator, expr } => {
                write!(self.buff, "{} ", operator.span.lexeme)?;
                self.visit(expr)?;
                Ok(())
            }

            AstNode::PadRead { x, y } => {
                write!(self.buff, "__read ")?;
                self.visit(x)?;
                write!(self.buff, ", ")?;
                self.visit(y)?;
                Ok(())
            }

            AstNode::FormalParam {
                identifier,
                param_type,
                index,
            } => {
                write!(
                    self.buff,
                    "{}: {}",
                    identifier.span.lexeme, param_type.span.lexeme
                )?;

                if let Some(index) = index {
                    write!(self.buff, "[{}]", index)?;
                }

                Ok(())
            }

            AstNode::PadRandI { upper_bound } => {
                write!(self.buff, "__randi ")?;
                self.visit(upper_bound)?;
                Ok(())
            }

            AstNode::FunctionCall { identifier, args } => {
                write!(self.buff, "{}(", identifier.span.lexeme)?;

                if !args.is_empty() {
                    for arg in args.iter().take(args.len() - 1) {
                        self.visit(arg)?;
                        write!(self.buff, ", ")?;
                    }

                    if let Some(last) = args.last() {
                        self.visit(last)?;
                    }
                }

                write!(self.buff, ")")?;

                Ok(())
            }

            AstNode::Identifier { token } => {
                write!(self.buff, "{}", token.span.lexeme)?;
                Ok(())
            }

            AstNode::IntLiteral(token) => {
                write!(self.buff, "{}", token.span.lexeme)?;
                Ok(())
            }

            AstNode::FloatLiteral(token) => {
                write!(self.buff, "{}", token.span.lexeme)?;
                Ok(())
            }

            AstNode::BoolLiteral(token) => {
                write!(self.buff, "{}", token.span.lexeme)?;
                Ok(())
            }

            AstNode::ColourLiteral(token) => {
                write!(self.buff, "{}", token.span.lexeme)?;
                Ok(())
            }
            AstNode::PadWidth => {
                write!(self.buff, "__width")?;
                Ok(())
            }
            AstNode::PadHeight => {
                write!(self.buff, "__height")?;
                Ok(())
            }
            AstNode::ActualParams { params: _ } => Ok(()),
            AstNode::EndOfFile => Ok(()),
        }
    }
}
