use crate::core::Token;

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
    Identifier {
        token: Token,
    },
    Block {
        statements: Vec<AstNode>,
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
    PadWidth,
    PadRandI {
        upper_bound: Ast,
    },
    PadHeight,
    PadRead {
        first: Ast,
        second: Ast,
    },
    IntLiteral(Token),
    FloatLiteral(Token),
    BoolLiteral(Token),
    ColourLiteral(Token),
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
    Print {
        expression: Box<AstNode>,
    },
    Assignment {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
    EndOfFile,
    PadClear {
        expr: Ast,
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
        match node {
            AstNode::Program { statements } => {
                for statement in statements {
                    self.visit(statement);
                    println!("\n");
                }
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
                print!(";");
            }

            AstNode::Delay { expression } => {
                print!("__delay ");
                self.visit(expression);
                print!(";");
            }

            AstNode::Print { expression } => {
                print!("__print ");
                self.visit(expression);
                print!(";");
            }

            AstNode::If {
                condition,
                if_true,
                if_false,
            } => {
                print!("if (");
                self.visit(condition);
                print!(") ");
                self.visit(if_true);
            }

            AstNode::Assignment {
                identifier,
                expression,
            } => {
                self.visit(identifier);
                print!(" = ");
                self.visit(expression);
                print!(";");
            }

            AstNode::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                print!("for (");

                // This is to avoid printing newline after the variable declaration in a for loop
                if let Some(AstNode::VarDec {
                    identifier,
                    var_type,
                    expression,
                }) = initializer.as_ref()
                {
                    print!("let ");
                    self.visit(identifier);
                    if let Some(var_type) = var_type {
                        print!(": {} = ", var_type.span.lexeme);
                    }
                    self.visit(expression);
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
                print!(") ");
                self.visit(body);
            }

            AstNode::Return { expression } => {
                print!("return ");
                self.visit(expression);
                print!(";");
            }

            AstNode::Block { statements } => {
                println!("\n{}{{", "  ".repeat(self.tab_level));
                self.tab_level += 1;
                for statement in statements {
                    print!("{}", "  ".repeat(self.tab_level));
                    self.visit(statement);
                    println!();
                }
                self.tab_level -= 1;
                print!("{}}}", "  ".repeat(self.tab_level));
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

            AstNode::While { condition, body } => {
                print!("while (");
                let prev_tab_level = self.tab_level;
                self.tab_level = 0;
                self.visit(condition);
                self.tab_level = prev_tab_level;
                print!(") ");
                self.visit(body);
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
                print!(";")
            }

            AstNode::PadClear { expr } => {
                print!("__clear ");

                self.visit(expr);

                print!(";")
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
                print!(";")
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
                print!("__read ");
                self.visit(first);
                print!(", ");
                self.visit(second);
            }

            AstNode::FormalParam {
                identifier,
                param_type,
            } => {
                self.visit(identifier);
                print!(": {}", param_type.span.lexeme);
            }

            AstNode::PadRandI { upper_bound } => {
                print!("__randi ");

                self.visit(upper_bound);
            }

            AstNode::FunctionCall { identifier, args } => {
                self.visit(identifier);
                print!("(");

                let (args, last) = args.split_at(args.len() - 1);

                for arg in args {
                    self.visit(arg);
                    print!(", ");
                }

                if let Some(last) = last.first() {
                    self.visit(last);
                }

                print!(")");
            }

            AstNode::Identifier { token } => {
                print!("{}", token.span.lexeme);
            }

            AstNode::IntLiteral(token) => {
                print!("{}", token.span.lexeme);
            }

            AstNode::FloatLiteral(token) => {
                print!("{}", token.span.lexeme);
            }

            AstNode::BoolLiteral(token) => {
                print!("{}", token.span.lexeme);
            }

            AstNode::ColourLiteral(token) => {
                print!("{}", token.span.lexeme);
            }
            AstNode::PadWidth => {
                print!("__width");
            }
            AstNode::PadHeight => {
                print!("__height");
            }
            AstNode::EndOfFile => {}
            AstNode::ActualParams { params } => todo!(),
        }
    }
}
