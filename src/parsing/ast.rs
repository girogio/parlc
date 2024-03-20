use std::fmt::Display;

use crate::core::{DataTypes, Token, TokenKind};

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

impl Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementType::Let => write!(f, "Let"),
            StatementType::If => write!(f, "If"),
            StatementType::While => write!(f, "While"),
            StatementType::Function => write!(f, "Function"),
            StatementType::Return => write!(f, "Return"),
            StatementType::Print { expression } => write!(f, "Print"),
            StatementType::Delay => write!(f, "Delay"),
            StatementType::PixelR => write!(f, "PixelR"),
            StatementType::Pixel => write!(f, "Pixel"),
        }
    }
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
    PadRandI,
    PadHeight,
    PadRead,
    IntLiteral(i32),
    FloatLiteral(String),
    BoolLiteral(bool),
    ColourLiteral(String),
    FunctionCall {
        identifier: Ast,
        args: Ast,
    },
    ActualParams {
        params: Vec<Ast>,
    },
}

pub type Ast = Box<AstNode>;

impl TryFrom<Token> for AstNode {
    type Error = crate::utils::errors::ParseError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::Identifier => Ok(AstNode::Identifier { token }),
            _ => Err(crate::utils::errors::ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: token,
                file: file!(),
                line: line!(),
                col: column!(),
            }),
        }
    }
}

pub trait Visitor {
    fn visit(&mut self, node: &AstNode);
}

pub struct AstPrinter {
    tab_level: usize,
}

fn print_node(node: &AstNode) {
    println!("{:?}", node);
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { tab_level: 0 }
    }
}

impl Visitor for AstPrinter {
    fn visit(&mut self, node: &AstNode) {
        print!("{}", "  ".repeat(self.tab_level));
        match node {
            AstNode::Program { statements } => {
                println!("Program");
                self.tab_level += 1;
                for statement in statements {
                    self.visit(statement);
                }
                self.tab_level -= 1;
            }

            AstNode::Identifier { token } => {
                println!("Identifier(\"{}\")", token.span.lexeme);
            }

            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                self.visit(identifier);
                println!("Type: {:?}", var_type.kind);
                self.visit(expression);
            }

            AstNode::Block { statements } => {
                println!("Block");
                for statement in statements {
                    self.visit(statement);
                }
            }

            AstNode::Expression {
                casted_type,
                bin_op,
            } => {
                print!("Expression casted to ");
                if let Some(casted_type) = casted_type {
                    println!("{}", casted_type.span.lexeme);
                }
                self.tab_level += 1;
                self.visit(bin_op);
                self.tab_level -= 1;
            }

            AstNode::Statement { kind } => {
                println!("{kind}");
                match kind {
                    StatementType::Let => println!("Let"),
                    StatementType::If => println!("If"),
                    StatementType::While => println!("While"),
                    StatementType::Function => println!("Function"),
                    StatementType::Return => println!("Return"),
                    StatementType::Print { expression } => {
                        self.tab_level += 1;
                        self.visit(expression);
                        self.tab_level -= 1;
                    }
                    StatementType::Delay => println!("Delay"),
                    StatementType::PixelR => println!("PixelR"),
                    StatementType::Pixel => {
                        println!("Pixel");
                        // self.visit(expression);
                    }
                }
            }

            AstNode::BinOp {
                left,
                operator,
                right,
            } => {
                println!("Operator: {:?}", operator.kind);
                self.tab_level += 1;
                self.visit(left);
                self.visit(right);
                self.tab_level -= 1;
            }

            AstNode::UnaryOp { operator, expr } => {
                println!("Operator: {:?}", operator.kind);
                self.tab_level += 1;
                self.visit(expr);
                self.tab_level -= 1;
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
