use crate::{core::Token, lexing::Lexer};

use super::ast::AstNode;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next: usize,
    root: AstNode,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            next: 0,
            root: AstNode::Program { statements: vec![] },
        }
    }

    pub fn parse(&mut self) -> &AstNode {
        self.root = self.parse_program();
        &self.root
    }

    fn parse_program(&mut self) -> AstNode {
        let mut statements = vec![];
        while self.current < self.tokens.len() {
            statements.push(self.parse_statement());
        }
        AstNode::Program { statements }
    }


    fn parse_statement(&mut self) -> AstNode {
        let kind = self.tokens[self.current].kind.clone();
        self.current += 1;

        AstNode::Statement {
            kind,
            expression: Box::new(AstNode::Empty),
        }
    }
}
