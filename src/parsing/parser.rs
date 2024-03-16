use crate::{
    core::{Token, TokenKind},
    utils::{errors::ParseError, Result},
};

use super::ast::{AstNode, StatementType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    root: AstNode,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
            root: AstNode::Program { statements: vec![] },
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn parse(&mut self) -> Result<&AstNode> {
        for token in &self.tokens {
            println!("{:?}", token);
        }
        println!();

        self.root = self.parse_program()?;
        Ok(&self.root)
    }

    fn parse_program(&mut self) -> Result<AstNode> {
        let mut statements = vec![];
        while self.current < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }
        Ok(AstNode::Program { statements })
    }

    fn parse_statement(&mut self) -> Result<AstNode> {
        let current_token_kind = self.current_token().kind.clone();
        self.current += 1;

        match current_token_kind {
            TokenKind::Let => self.parse_assignment(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::Print => self.parse_print_statement(),
            _ => Ok(AstNode::Empty),
        }
    }

    fn parse_identifier(&mut self) -> Result<AstNode> {
        if matches!(self.current_token().kind, TokenKind::Identifier(_)) {
            Ok(AstNode::Identifier {
                kind: self.consume().clone(),
            })
        } else {
            Err(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier("".to_string()),
                found: self.current_token().clone(),
            }
            .into())
        }
    }

    fn parse_assignment(&mut self) -> Result<AstNode> {
        let identifier = self.parse_identifier()?;
        self.consume_if(TokenKind::Colon)?;
        let var_type = self.consume().clone();
        self.consume_if(TokenKind::Equals)?;
        // let expression = self.parse_expression()?;
        self.consume_if(TokenKind::Semicolon)?;
        Ok(AstNode::VarDec {
            identifier: Box::from(identifier),
            var_type,
            expression: Box::new(AstNode::Empty),
        })
    }

    fn parse_print_statement(&mut self) -> Result<AstNode> {
        // let expression = self.parse_expression()?;
        Ok(AstNode::Statement {
            kind: StatementType::Print {
                expression: Box::new(AstNode::Empty),
            },
        })
    }

    fn parse_block(&mut self) -> Result<AstNode> {
        let mut statements = vec![];
        while self.current_token().kind != TokenKind::RBrace {
            if self.current_token().kind == TokenKind::EndOfFile {
                return Err(ParseError::UnclosedBlock.into());
            }
            statements.push(self.parse_statement()?);
        }
        self.current += 1;
        Ok(AstNode::Block { statements })
    }

    fn consume(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn consume_if(&mut self, kind: TokenKind) -> Result<&Token> {
        if self.current_token().kind == kind {
            Ok(self.consume())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: kind,
                found: self.current_token().clone(),
            }
            .into())
        }
    }
}
