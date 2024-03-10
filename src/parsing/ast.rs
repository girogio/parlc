use crate::core::{DataTypes, TokenKind};

#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    Block {
        statements: Vec<AstNode>,
        compound_statement: Ast,
    },
    Statement {
        kind: TokenKind,
        expression: Box<AstNode>,
    },
    Expression {
        left: Box<AstNode>,
        operator: TokenKind,
        right: Box<AstNode>,
    },
    SimpleExpression {
        left: Box<AstNode>,
        operator: TokenKind,
        right: Box<AstNode>,
    },
    Term {
        left: Ast,
        operator: TokenKind,
        right: Ast,
    },
    Factor {
        kind: TokenKind,
        value: DataTypes,
    },
    Empty,
}

pub type Ast = Box<AstNode>;

pub trait Visitor {
    fn visit_program(&self, program: &AstNode);
    fn visit_statement(&self, statement: &AstNode);
    fn visit_block(&self, block: &AstNode);
    fn visit_expression(&self, expression: &AstNode);
    fn visit_simple_expression(&self, simple_expression: &AstNode);
    fn visit_term(&self, term: &AstNode);
    fn visit_factor(&self, factor: &AstNode);
    fn visit_empty(&self, empty: &AstNode);
}

pub struct AstPrinter;

impl Visitor for AstPrinter {
    fn visit_program(&self, program: &AstNode) {
        match program {
            AstNode::Program { statements } => {
                for statement in statements {
                    self.visit_statement(statement);
                }
            }
            _ => panic!("Expected Program node"),
        }
    }

    fn visit_statement(&self, statement: &AstNode) {
        match statement {
            AstNode::Statement { kind, expression } => {
                println!("Statement: {:?}", kind);
                self.visit_expression(expression);
            }
            _ => panic!("Expected Statement node"),
        }
    }

    fn visit_block(&self, block: &AstNode) {
        match block {
            AstNode::Block {
                statements,
                compound_statement,
            } => {
                for statement in statements {
                    self.visit_statement(statement);
                }
                self.visit_expression(compound_statement);
            }
            _ => panic!("Expected Block node"),
        }
    }

    fn visit_expression(&self, expression: &AstNode) {
        match expression {
            AstNode::Expression {
                left,
                operator,
                right,
            } => {
                self.visit_expression(left);
                println!("Expression: {:?}", operator);
                self.visit_expression(right);
            }
            _ => println!("{:?}", expression),
        }
    }

    fn visit_simple_expression(&self, simple_expression: &AstNode) {
        match simple_expression {
            AstNode::SimpleExpression {
                left,
                operator,
                right,
            } => {
                self.visit_simple_expression(left);
                println!("SimpleExpression: {:?}", operator);
                self.visit_simple_expression(right);
            }
            _ => panic!("Expected SimpleExpression node"),
        }
    }

    fn visit_term(&self, term: &AstNode) {
        match term {
            AstNode::Term {
                left,
                operator,
                right,
            } => {
                self.visit_term(left);
                println!("Term: {:?}", operator);
                self.visit_term(right);
            }
            _ => panic!("Expected Term node"),
        }
    }

    fn visit_factor(&self, factor: &AstNode) {
        match factor {
            AstNode::Factor { kind, value } => {
                println!("Factor: {:?}", kind);
                println!("Value: {:?}", value);
            }
            _ => panic!("Expected Factor node"),
        }
    }

    fn visit_empty(&self, _empty: &AstNode) {
        println!("Empty");
    }
}

// type AstNode
//  = AstNode
// Node;

// // pub struct AstNode
// ExpressionNode {
// //     pub left: Box<Option<AstNode
// SimpleExpressionNode>>,
// //     pub operator: AstNode
// OperatorNode,
// //     pub right: Box<AstNode
// SimpleExpressionNode>,
// // }

// // pub struct AstNode
// SimpleExpressionNode {
// //     pub left: AstNode
// TermNode,
// //     pub operator: AstNode
// OperatorNode,
// //     pub right: u32,
// // }
