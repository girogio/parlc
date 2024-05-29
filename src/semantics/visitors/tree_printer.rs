use crate::core::{AstNode, Visitor};
use crate::utils::Result;

pub struct TreePrinter {
    tab_level: usize,
}

impl TreePrinter {
    pub fn new() -> Self {
        Self { tab_level: 0 }
    }

    fn tab(&self) -> String {
        "  ".repeat(self.tab_level)
    }
    fn print_tab(&self) {
        print!("{}", self.tab());
    }
}

impl Visitor<Result<()>> for TreePrinter {
    fn visit(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program { statements } => {
                println!("Program");
                self.tab_level += 1;
                for statement in statements {
                    self.print_tab();
                    self.visit(statement)?;
                    println!();
                }
                self.tab_level -= 1;
                println!();
                Ok(())
            }

            AstNode::ArrayAccess { identifier, index } => {
                println!("ArrayAccess");
                self.tab_level += 1;
                self.print_tab();
                println!("Identifier: {}", identifier);
                self.print_tab();
                print!("Index: ");
                self.visit(index)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::VarDecArray {
                identifier,
                element_type,
                size,
                elements,
            } => {
                println!("VarDecArray");
                self.tab_level += 1;
                self.print_tab();
                println!("Identifier: {}", identifier);
                self.print_tab();
                println!("Element Type: {}", element_type);
                self.print_tab();
                println!("Size: {}", size);
                self.print_tab();
                println!("Elements: ");
                self.tab_level += 1;
                self.print_tab();
                for element in elements {
                    self.visit(element)?;
                    print!(", ");
                }
                self.tab_level -= 1;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                println!("VarDec");
                self.tab_level += 1;
                self.print_tab();
                println!("Identifier: {}", identifier);
                self.print_tab();
                println!("Type: {}", var_type);
                self.print_tab();
                print!("Expression: ");
                self.visit(expression)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Delay { expression } => {
                println!("Delay");
                self.tab_level += 1;
                self.print_tab();
                print!("Expression: ");
                self.visit(expression)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Print { expression } => {
                println!("Print");
                self.tab_level += 1;
                self.print_tab();
                print!("Expression: ");
                self.visit(expression)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                println!("If");
                self.tab_level += 1;
                self.print_tab();
                print!("Condition: ");
                self.visit(condition)?;
                println!();
                self.print_tab();
                print!("If True: ");
                self.visit(if_true)?;
                if let Some(if_false) = if_false {
                    self.print_tab();
                    print!("If False: ");
                    self.visit(if_false)?;
                }
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Assignment {
                identifier,
                index,
                expression,
            } => {
                println!("Assignment");
                self.tab_level += 1;
                self.print_tab();
                println!("Identifier: {}", identifier);
                self.print_tab();
                print!("Expression: ");
                self.visit(expression)?;
                println!();
                if let Some(index) = index {
                    self.print_tab();
                    print!("Index: ");
                    self.visit(index)?;
                    println!();
                }
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                println!("For");
                self.tab_level += 1;
                self.print_tab();
                print!("Initializer: ");
                if let Some(initializer) = initializer {
                    self.visit(initializer)?;
                    println!();
                } else {
                    println!("None");
                }
                self.print_tab();
                print!("Condition: ");
                self.visit(condition)?;
                println!();
                self.print_tab();
                print!("Increment: ");
                if let Some(increment) = increment {
                    self.tab_level += 1;
                    println!();
                    self.print_tab();
                    self.visit(increment)?;
                    self.tab_level -= 1;
                } else {
                    print!("None");
                }
                println!();
                self.print_tab();
                print!("Body: ");
                self.visit(body)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Return { expression } => {
                println!("Return");
                self.tab_level += 1;
                self.print_tab();
                print!("Expression: ");
                self.visit(expression)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Block { statements } => {
                println!("Block");
                self.tab_level += 1;
                for statement in statements {
                    self.print_tab();
                    self.visit(statement)?;
                    println!();
                }
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::Expression {
                casted_type,
                expr: bin_op,
            } => {
                self.tab_level += 1;
                self.visit(bin_op)?;
                if let Some(casted_type) = casted_type {
                    print!(" as {}", casted_type);
                }
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::SubExpression { bin_op } => {
                print!("(");
                self.visit(bin_op)?;
                print!(")");
                Ok(())
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                println!("FunctionDecl");
                self.tab_level += 1;
                self.print_tab();
                println!("Identifier: {}", identifier.span.lexeme);
                self.print_tab();
                print!("Params: ");
                for param in params {
                    self.visit(param)?;
                    print!(", ");
                }
                println!();
                self.print_tab();
                println!("Return Type: {}", return_type);
                self.print_tab();
                print!("Block: ");
                self.visit(block)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::While { condition, body } => {
                println!("While");
                self.tab_level += 1;
                self.print_tab();
                print!("Condition: ");
                self.visit(condition)?;
                self.print_tab();
                print!("Body: ");
                self.visit(body)?;
                self.tab_level -= 1;
                Ok(())
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                print!("__write ");
                self.visit(loc_x)?;
                print!(", ");
                self.visit(loc_y)?;
                print!(", ");
                self.visit(colour)?;
                print!(";");
                Ok(())
            }

            AstNode::PadClear { expr } => {
                print!("__clear ");
                self.visit(expr)?;
                print!(";");
                Ok(())
            }

            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => {
                print!("__write_box ");
                self.visit(loc_x)?;
                print!(", ");
                self.visit(loc_y)?;
                print!(", ");
                self.visit(width)?;
                print!(", ");
                self.visit(height)?;
                print!(", ");
                self.visit(colour)?;
                print!(";");
                Ok(())
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                print!("(");
                self.visit(left)?;
                print!(" {} ", operator);
                self.visit(right)?;
                print!(")");
                Ok(())
            }

            AstNode::UnaryOp { operator, expr } => {
                print!("{}(", operator);
                self.visit(expr)?;
                print!(")");
                Ok(())
            }

            AstNode::PadRead { x, y } => {
                print!("__read ");
                self.visit(x)?;
                print!(", ");
                self.visit(y)?;
                Ok(())
            }

            AstNode::FormalParam {
                identifier,
                param_type,
                index,
            } => {
                print!("{}: {}", identifier.span.lexeme, param_type.span.lexeme);
                if let Some(index) = index {
                    print!("[{}]", index);
                }
                Ok(())
            }

            AstNode::PadRandI { upper_bound } => {
                print!("__randi ");
                self.visit(upper_bound)?;
                Ok(())
            }

            AstNode::FunctionCall { identifier, args } => {
                print!("{}(", identifier.span.lexeme);

                if !args.is_empty() {
                    let (args, last) = args.split_at(args.len() - 1);

                    for arg in args {
                        self.visit(arg)?;
                        print!(", ");
                    }

                    if let Some(last) = last.first() {
                        self.visit(last)?;
                    }
                }

                print!(")");
                Ok(())
            }

            AstNode::Identifier { token } => {
                print!("{}", token.span.lexeme);
                Ok(())
            }

            AstNode::IntLiteral(token) => {
                print!("{}", token.span.lexeme);
                Ok(())
            }

            AstNode::FloatLiteral(token) => {
                print!("{}", token.span.lexeme);
                Ok(())
            }

            AstNode::BoolLiteral(token) => {
                print!("{}", token.span.lexeme);
                Ok(())
            }

            AstNode::ColourLiteral(token) => {
                print!("{}", token.span.lexeme);
                Ok(())
            }
            AstNode::PadWidth => {
                print!("__width");
                Ok(())
            }
            AstNode::PadHeight => {
                print!("__height");
                Ok(())
            }

            AstNode::ActualParams { params: _ } => Ok(()),

            AstNode::EndOfFile => Ok(()),
        }
    }
}
