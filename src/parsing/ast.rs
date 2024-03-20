use crate::core::Token;

#[derive(Debug)]
pub enum StatementType {
    Let,
    If,
    While,
    Function,
    Return,
    Print {
        expression: Ast,
    },
    Delay,
    PixelR,
    Pixel,
    Write {
        expression: Ast,
    },
    Assignment {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
}

#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    VarDec {
        identifier: Ast,
        var_type: Option<Token>,
        expression: Ast,
    },
    DelayStatement {
        expression: Ast,
    },
    PixelStatement {
        expression: Ast,
    },
    ReturnStatement {
        expression: Ast,
    },
    IfStatement {
        condition: Ast,
        then_branch: Ast,
        else_branch: Ast,
    },
    ForStatement {
        initializer: Ast,
        condition: Ast,
        increment: Ast,
        body: Ast,
    },
    WhileStatement {
        condition: Ast,
        body: Ast,
    },
    Identifier {
        token: Token,
    },
    Block {
        statements: Vec<AstNode>,
    },
    Statement {
        kind: StatementType,
    },
    Expression {
        casted_type: Option<Token>,
        bin_op: Ast,
    },
    UnaryOp {
        operator: Token,
        expr: Ast,
    },
    BinOp {
        left: Ast,
        operator: Token,
        right: Ast,
    },
    Empty,
    PadWidth,
    PadRandI {
        upper_bound: Ast,
    },
    PadHeight,
    PadRead {
        first: Ast,
        second: Ast,
    },
    IntLiteral(i32),
    FloatLiteral(String),
    BoolLiteral(bool),
    ColourLiteral(String),
    FunctionCall {
        identifier: Ast,
        args: Vec<Ast>,
    },
    ActualParams {
        params: Vec<Ast>,
    },
    Delay {
        expression: Box<AstNode>,
    },
    Return {
        expression: Box<AstNode>,
    },
    PadWriteBox {
        loc_x: Box<AstNode>,
        loc_y: Box<AstNode>,
        width: Box<AstNode>,
        height: Box<AstNode>,
        colour: Box<AstNode>,
    },
    PadWrite {
        loc_x: Box<AstNode>,
        loc_y: Box<AstNode>,
        colour: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        if_true: Box<AstNode>,
        if_false: Option<Box<AstNode>>,
    },
    For {
        initializer: Box<Option<AstNode>>,
        condition: Box<AstNode>,
        increment: Box<Option<AstNode>>,
        body: Box<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    FormalParam {
        identifier: Box<AstNode>,
        param_type: Token,
    },
    FunctionDecl {
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        return_type: Token,
        block: Box<AstNode>,
    },
}

pub type Ast = Box<AstNode>;

pub trait Visitor {
    fn visit(&mut self, node: &AstNode);
}

pub struct AstPrinter {
    tab_level: usize,
    reset: bool,
}

fn print_node(node: &AstNode) {
    print!("{:?}", node);
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {
            tab_level: 0,
            reset: false,
        }
    }

    fn reset(&mut self) -> usize {
        let prev_tab_level = self.tab_level;
        self.tab_level = 0;
        prev_tab_level
    }

    fn restore(&mut self, prev_tab_level: usize) {
        self.tab_level = prev_tab_level;
    }
}

impl Visitor for AstPrinter {
    fn visit(&mut self, node: &AstNode) {
        if !self.reset {
            print!("{}", "  ".repeat(self.tab_level));
        } else {
            self.reset = false;
        }
        match node {
            AstNode::Program { statements } => {
                for statement in statements {
                    self.visit(statement);
                }
            }

            AstNode::Identifier { token } => {
                print!("{}", token.span.lexeme);
            }

            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                print!("let ");
                self.visit(identifier);
                match var_type {
                    Some(var_type) => print!(": {} = ", var_type.span.lexeme),
                    None => println!(": n/a"),
                }
                self.visit(expression);
            }

            AstNode::Block { statements } => {
                println!("{}{{", "  ".repeat(self.tab_level));
                self.tab_level += 1;
                for statement in statements {
                    self.visit(statement);
                }
                self.tab_level -= 1;
                println!("\n{}}}", "  ".repeat(self.tab_level));
            }

            AstNode::Expression {
                casted_type,
                bin_op,
            } => {
                self.visit(bin_op);
                match casted_type {
                    Some(casted_type) => print!(" as {}", casted_type.span.lexeme),
                    None => {}
                }
            }

            AstNode::FunctionDecl {
                identifier,
                params,
                return_type,
                block,
            } => {
                print!("fn ");
                let a = self.reset();
                self.visit(identifier);
                print!("(");
                for param in params {
                    self.visit(param);
                }
                print!(") -> {} ", return_type.span.lexeme);
                self.restore(a);
                self.visit(block);
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                print!("for (");
                match initializer.as_ref() {
                    Some(initializer) => {
                        self.visit(initializer);
                    }
                    None => {}
                }
                print!("; ");
                self.visit(condition);
                print!("; ");
                match increment.as_ref() {
                    Some(increment) => {
                        self.visit(increment);
                    }
                    None => {}
                }
                println!(") ");
                self.visit(body);
            }

            AstNode::While { condition, body } => {
                print!("while (");
                let prev_tab_level = self.tab_level;
                self.tab_level = 0;
                self.visit(condition);
                self.tab_level = prev_tab_level;
                println!(") ");
                self.visit(body);
            }

            AstNode::Statement { kind } => {
                self.tab_level += 1;
                match kind {
                    StatementType::Write { expression } => {
                        print!("__write ");
                        self.visit(expression);
                    }
                    StatementType::Assignment {
                        identifier,
                        expression,
                    } => {
                        self.visit(identifier);
                        print!(" = ");
                        let prev_tab_level = self.tab_level;
                        self.tab_level = 0;
                        self.visit(expression);
                        print!("; ");
                        self.tab_level = prev_tab_level;
                    }
                    StatementType::Print { expression } => {
                        print!("__print ");
                        let prev_tab_level = self.tab_level;
                        self.tab_level = 0;
                        self.visit(expression);
                        self.tab_level = prev_tab_level;
                    }
                    _ => print_node(node),
                }
                self.tab_level -= 1;
            }

            AstNode::PadWrite {
                loc_x,
                loc_y,
                colour,
            } => {
                print!("__write ");
                self.visit(loc_x);
                print!(", ");
                self.visit(loc_y);
                print!(", ");
                self.visit(colour);
            }

            AstNode::PadWriteBox {
                loc_x,
                loc_y,
                width,
                height,
                colour,
            } => {
                print!("__write_box ");
                self.visit(loc_x);
                print!(", ");
                self.visit(loc_y);
                print!(", ");
                self.visit(width);
                print!(", ");
                self.visit(height);
                print!(", ");
                self.visit(colour);
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                self.visit(left);
                print!(" {} ", operator.span.lexeme);
                self.visit(right);
            }

            AstNode::UnaryOp { operator, expr } => {
                print!("{} ", operator.span.lexeme);
                self.visit(expr);
            }

            AstNode::PadRead { first, second } => {
                println!("Read: ");
                self.visit(first);
                self.visit(second);
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                print!("if (");
                let prev_tab_level = self.tab_level;
                self.tab_level = 0;
                self.visit(condition);
                self.tab_level = prev_tab_level;
                println!(") ");
                self.visit(if_true);
            }

            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                self.visit(identifier);
                print!(": {}, ", param_type.span.lexeme);
            }

            AstNode::PadRandI { upper_bound } => {
                println!("Random Int: ");
                self.tab_level += 1;
                println!("{}Upper Bound: ", "  ".repeat(self.tab_level));
                self.tab_level += 1;
                self.visit(upper_bound);
                self.tab_level -= 2;
            }

            AstNode::FunctionCall { identifier, args } => {
                println!("Function Call: ");
                self.visit(identifier);
                println!("{}Arguments: ", "  ".repeat(self.tab_level));
                self.tab_level += 1;
                for arg in args {
                    self.visit(arg);
                }
                self.tab_level -= 1;
            }

            AstNode::Empty => {
                print_node(node);
            }
            AstNode::DelayStatement { expression } => {
                println!("Delay: ");
                self.visit(expression);
            }
            AstNode::PixelStatement { expression } => todo!(),
            AstNode::ReturnStatement { expression } => todo!(),
            AstNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            AstNode::ForStatement {
                initializer,
                condition,
                increment,
                body,
            } => todo!(),
            AstNode::WhileStatement { condition, body } => todo!(),
            AstNode::PadWidth => todo!(),
            AstNode::PadHeight => todo!(),
            AstNode::ActualParams { params } => todo!(),
            AstNode::Delay { expression } => {
                println!("Delay: ");
                self.visit(expression);
            }
            AstNode::Return { expression } => {
                print!("return ");
                let a = self.reset();
                self.visit(expression);
                print!(";");
                self.restore(a);
            }
            _ => print_node(node),
        }
    }
}
