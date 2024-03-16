use crate::core::{DataTypes, Token};

#[derive(Debug)]
pub enum StatementType {
    Let,
    If,
    While,
    Function,
    Return,
    Print { expression: Ast },
    Delay,
    PixelR,
    Pixel,
}

#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    VarDec {
        identifier: Ast,
        var_type: Token,
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
        kind: Token,
    },
    Block {
        statements: Vec<AstNode>,
    },
    Statement {
        kind: StatementType,
    },
    Expression {
        left: Ast,
        operator: Token,
        right: Ast,
    },
    SimpleExpression {
        left: Ast,
        operator: Token,
        right: Ast,
    },
    Term {
        left: Ast,
        operator: Token,
        right: Ast,
    },
    Factor {
        kind: Token,
        value: DataTypes,
    },
    Empty,
}

pub type Ast = Box<AstNode>;

pub trait Visitor {
    fn visit(&self, node: &AstNode);
}

pub struct AstPrinter;

fn print_node(node: &AstNode) {
    println!("{:?}", node);
}

impl Visitor for AstPrinter {
    fn visit(&self, node: &AstNode) {
        match node {
            AstNode::Program { statements } => {
                for statement in statements {
                    self.visit(statement);
                }
            }
            AstNode::Identifier { kind } => {
                println!("Identifier: {}", kind.kind);
            }
            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                self.visit(identifier);
                println!("Type: {}", var_type.kind);
                // self.visit(expression);
            }
            AstNode::Block { statements } => {
                println!("Block");
                for statement in statements {
                    self.visit(statement);
                }
            }
            AstNode::Statement { kind } => {
                // print_node(node);
                match kind {
                    StatementType::Let => println!("Let"),
                    StatementType::If => println!("If"),
                    StatementType::While => println!("While"),
                    StatementType::Function => println!("Function"),
                    StatementType::Return => println!("Return"),
                    StatementType::Print { expression } => {
                        // Remove the println! statement for the StatementType::Print variant
                        // println!("Print"),
                        println!("Print");
                        self.visit(expression);
                    }
                    StatementType::Delay => println!("Delay"),
                    StatementType::PixelR => println!("PixelR"),
                    StatementType::Pixel => {
                        println!("Pixel");
                        // self.visit(expression);
                    }
                }
            }
            AstNode::Expression {
                left,
                operator,
                right,
            } => {
                self.visit(left);
                print_node(node);
                self.visit(right);
            }
            AstNode::SimpleExpression {
                left,
                operator,
                right,
            } => {
                self.visit(left);
                print_node(node);
                self.visit(right);
            }
            AstNode::Term {
                left,
                operator,
                right,
            } => {
                self.visit(left);
                print_node(node);
                self.visit(right);
            }
            AstNode::Factor { kind, value } => {
                print_node(node);
            }
            AstNode::Empty => {
                print_node(node);
            }
            _ => {
                print_node(node);
            }
        }
    }
}
