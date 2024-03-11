use crate::core::{DataTypes, Token};

#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    VarDec {
        identifier: Token,
        var_type: Token,
        expression: Ast,
    },
    Block {
        statements: Vec<AstNode>,
    },
    Statement {
        kind: Token,
        expression: Ast,
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
    fn visit_program(&self, program: &AstNode);
    fn visit_variable_declaration(&self, variable_declaration: &AstNode);
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
        println!("Program");
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
            AstNode::Block { statements } => {
                println!("Block");
                for statement in statements {
                    self.visit_statement(statement);
                }
                println!("EndBlock");
            }
            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                println!("VariableDeclaration");
                println!("\t {:?}", identifier.kind);
                println!("\t Type: {:?}", var_type);
                self.visit_expression(expression);
            }
            AstNode::Empty => {
                println!("Empty");
            }
            _ => panic!("Expected Statement or Block node"),
        }
    }

    fn visit_variable_declaration(&self, variable_declaration: &AstNode) {
        match variable_declaration {
            AstNode::VarDec {
                identifier,
                var_type,
                expression,
            } => {
                println!("VariableDeclaration: {:?}", identifier);
                println!("Type: {:?}", var_type);
                self.visit_expression(expression);
            }
            _ => panic!("Expected VariableDeclaration node"),
        }
    }

    fn visit_block(&self, block: &AstNode) {
        match block {
            AstNode::Block { statements } => {
                for statement in statements {
                    self.visit_statement(statement);
                }
            }
            _ => panic!("Expected Block node"),
        }
    }
    fn visit_expression(&self, expression: &AstNode) {
        println!("Expression: {:?}", expression);
    }
    fn visit_simple_expression(&self, simple_expression: &AstNode) {
        println!("SimpleExpression: {:?}", simple_expression);
    }
    fn visit_term(&self, term: &AstNode) {
        println!("Term: {:?}", term);
    }
    fn visit_factor(&self, factor: &AstNode) {
        println!("Factor: {:?}", factor);
    }
    fn visit_empty(&self, empty: &AstNode) {
        println!("Empty: {:?}", empty);
    }
}
