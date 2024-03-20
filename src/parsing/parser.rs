use crate::{
    core::{TextSpan, Token, TokenKind},
    utils::{
        errors::{Error, ParseError},
        Result,
    },
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

    fn current_token(&self) -> Token {
        let eof_token = Token::new(TokenKind::EndOfFile, TextSpan::new(0, 0, 0, 0, "\0"));
        self.tokens.get(self.current).unwrap_or(&eof_token).clone()
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn parse(&mut self) -> Result<&AstNode> {
        println!("Tokens: \n");
        for token in &self.tokens {
            println!("{}", token);
        }
        println!("\nParsing...\n ");

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
        let current_token = self.current_token();

        match &current_token.kind {
            TokenKind::Let => self.parse_var_decl(),
            TokenKind::Identifier => self.parse_identifier(),
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::Delay => todo!(),
            TokenKind::PadWrite => todo!(),
            TokenKind::PadWriteBox => todo!(),
            TokenKind::If => todo!(),
            TokenKind::For => todo!(),
            TokenKind::While => todo!(),
            TokenKind::Return => todo!(),
            TokenKind::Function => todo!(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::EndOfFile => Ok(AstNode::Empty),
            _ => Err(Error::Parse(ParseError::UnexpectedToken {
                expected: TokenKind::Invalid,
                file: file!(),
                line: line!(),
                col: column!(),
                found: current_token.clone(),
            })),
        }
    }

    fn parse_expression(&mut self) -> Result<AstNode> {
        let left = self.parse_simple_expr()?;

        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::As => {
                self.consume();
                let kind = self.consume_if(TokenKind::Type)?.clone();
                Ok(AstNode::Expression {
                    casted_type: Some(kind),
                    bin_op: Box::new(left),
                })
            }
            TokenKind::LessThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEqual
            | TokenKind::EqEq
            | TokenKind::NotEqual => {
                self.consume();
                let right = self.parse_simple_expr()?;

                Ok(AstNode::Expression {
                    casted_type: if let TokenKind::As = self.current_token().kind {
                        self.consume();
                        let kind = self.consume_if(TokenKind::Type)?.clone();
                        Some(kind)
                    } else {
                        None
                    },
                    bin_op: Box::new(AstNode::BinOp {
                        left: Box::new(left),
                        operator: curr,
                        right: Box::new(right),
                    }),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_simple_expr(&mut self) -> Result<AstNode> {
        let left = self.parse_term()?;

        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Or => {
                self.consume();
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator: curr,
                    right: Box::new(self.parse_term()?),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_sub_expr(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::LParen)?;
        let expr = self.parse_expression();
        self.consume_if(TokenKind::RParen)?;
        expr
    }

    fn parse_term(&mut self) -> Result<AstNode> {
        let left = self.parse_factor()?;
        let next_token = self.current_token().clone();

        match &next_token.kind {
            TokenKind::Multiply | TokenKind::Divide | TokenKind::And => {
                self.consume();
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator: next_token,
                    right: Box::new(self.parse_factor()?),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_factor(&mut self) -> Result<AstNode> {
        let next_token = self.current_token().clone();

        match &next_token.kind {
            TokenKind::Identifier => {
                let ident = self.parse_identifier()?;

                if self.current_token().kind == TokenKind::LParen {
                    self.consume();
                    if self.current_token().kind == TokenKind::RParen {
                        self.consume();
                        Ok(AstNode::FunctionCall {
                            identifier: Box::new(ident),
                            args: Box::new(AstNode::ActualParams { params: vec![] }),
                        })
                    } else {
                        let args = self.parse_actual_params()?;
                        self.consume_if(TokenKind::RParen)?;
                        Ok(AstNode::FunctionCall {
                            identifier: Box::new(ident),
                            args: Box::new(args),
                        })
                    }
                } else {
                    Ok(ident)
                }
            }
            TokenKind::PadWidth => self.parse_pad_width(),
            TokenKind::LParen => self.parse_sub_expr(),
            TokenKind::Minus | TokenKind::Not => self.parse_unary_expr(),
            TokenKind::PadRandI => self.parse_pad_rand_i(),
            TokenKind::IntLiteral(_)
            | TokenKind::FloatLiteral(_)
            | TokenKind::BoolLiteral(_)
            | TokenKind::ColourLiteral(_) => self.parse_literal(),
            TokenKind::PadHeight => self.parse_pad_height(),
            TokenKind::PadRead => self.parse_pad_read(),
            _ => Err(Error::Parse(ParseError::UnexpectedTokenList {
                expected: vec![TokenKind::Identifier],
                found: next_token.clone(),
                file: file!(),
                line: line!(),
                col: column!(),
            })),
        }
    }

    fn parse_identifier(&mut self) -> Result<AstNode> {
        let ident = self.consume_if(TokenKind::Identifier)?;

        Ok(AstNode::Identifier {
            token: ident.clone(),
        })
    }

    fn parse_pad_width(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadWidth)?;
        Ok(AstNode::PadWidth)
    }

    fn parse_pad_rand_i(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadRandI)?;
        Ok(AstNode::PadRandI)
    }

    fn parse_pad_height(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadHeight)?;
        Ok(AstNode::PadHeight)
    }

    fn parse_pad_read(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadRead)?;
        Ok(AstNode::PadRead)
    }

    fn parse_var_decl(&mut self) -> Result<AstNode> {
        let identifier = self.parse_identifier()?;
        self.consume_if(TokenKind::Colon)?;
        let var_type = self.consume().clone();
        self.consume_if(TokenKind::Equals)?;
        let expression = self.parse_expression()?;
        self.consume_if(TokenKind::Semicolon)?;
        Ok(AstNode::VarDec {
            identifier: Box::from(identifier),
            var_type,
            expression: Box::new(expression),
        })
    }

    fn parse_print_statement(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Print)?;
        let expression = self.parse_expression()?;
        Ok(AstNode::Statement {
            kind: StatementType::Print {
                expression: Box::new(expression),
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
                file: file!(),
                line: line!(),
                col: column!(),
            }
            .into())
        }
    }

    fn consume_if_any<T: Clone + IntoIterator<Item = TokenKind>>(
        &mut self,
        kinds: T,
    ) -> Result<Token> {
        let found = kinds
            .clone()
            .into_iter()
            .any(|kind| self.current_token().kind == kind);
        if found {
            Ok(self.consume().clone())
        } else {
            Err(ParseError::UnexpectedTokenList {
                expected: kinds.into_iter().collect(),
                found: self.current_token().clone(),
                file: file!(),
                line: line!(),
                col: column!(),
            }
            .into())
        }
    }

    fn parse_unary_expr(&mut self) -> Result<AstNode> {
        let operator = self.consume_if_any([TokenKind::Minus, TokenKind::Not])?;
        let expr = self.parse_expression()?;
        Ok(AstNode::UnaryOp {
            operator,
            expr: Box::new(expr),
        })
    }

    fn parse_literal(&mut self) -> Result<AstNode> {
        let token = self.consume().clone();

        match token.kind {
            TokenKind::IntLiteral(s) => Ok(AstNode::IntLiteral(s)),
            TokenKind::FloatLiteral(s) => Ok(AstNode::FloatLiteral(s)),
            TokenKind::BoolLiteral(s) => Ok(AstNode::BoolLiteral(s)),
            TokenKind::ColourLiteral(s) => Ok(AstNode::ColourLiteral(s)),
            _ => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Invalid,
                found: token.clone(),
                file: file!(),
                line: line!(),
                col: column!(),
            }
            .into()),
        }
    }

    fn parse_actual_params(&mut self) -> Result<AstNode> {
        let mut params = vec![Box::from(self.parse_expression()?)];

        if let TokenKind::Comma = self.current_token().kind {
            self.consume();
            while self.current_token().kind != TokenKind::RParen {
                params.push(Box::from(self.parse_expression()?));
                if let TokenKind::Comma = self.current_token().kind {
                    self.consume();
                    self.parse_actual_params()?;
                }
            }
            self.consume();
        }

        Ok(AstNode::ActualParams { params })
    }

    // fn parse_literal(&mut self) -> Result<AstNode> {
    //     let token = self.consume_
    // }
}
