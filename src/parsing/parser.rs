use std::path::{Path, PathBuf};

use crate::{
    core::{TextSpan, Token, TokenKind},
    utils::{
        errors::{Error, ParseError},
        Result,
    },
};

use super::ast::{Ast, AstNode};

pub struct Parser {
    tokens: Vec<Token>,
    source_file: PathBuf,
    current: usize,
    root: AstNode,
}

impl Parser {
    pub fn new(tokens: &[Token], source_file: &Path) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
            source_file: source_file.to_path_buf(),
            root: AstNode::Program { statements: vec![] },
        }
    }

    fn current_token(&self) -> Token {
        let eof_token = Token::new(TokenKind::EndOfFile, TextSpan::new(0, 0, 0, 0, "\0"));
        self.tokens.get(self.current).unwrap_or(&eof_token).clone()
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    pub fn parse(&mut self) -> Result<&AstNode> {
        self.root = self.parse_program()?;
        Ok(&self.root)
    }

    fn parse_program(&mut self) -> Result<AstNode> {
        let mut statements = vec![];
        while self.current < self.tokens.len() {
            let next_statement = self.parse_statement()?;

            if let AstNode::EndOfFile = next_statement {
                break;
            }

            statements.push(next_statement);
        }
        Ok(AstNode::Program { statements })
    }

    fn parse_statement(&mut self) -> Result<AstNode> {
        let current_token = self.current_token();

        match &current_token.kind {
            TokenKind::Let => self.parse_var_decl(),
            TokenKind::Identifier => {
                let next_tok = self.peek_token();
                match next_tok {
                    Some(tok) => match tok.kind {
                        TokenKind::Equals => {
                            let a = self.parse_assignment_statement();
                            self.consume_if(TokenKind::Semicolon)?;
                            a
                        }
                        _ => self.parse_identifier(),
                    },
                    None => self.parse_identifier(),
                }
            }
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::Delay => self.parse_delay(),
            TokenKind::PadWrite => self.parse_write(),
            TokenKind::PadWriteBox => self.parse_write_box(),
            TokenKind::PadClear => self.parse_clear_statement(),
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::While => self.parse_while(),
            TokenKind::Function => self.parse_function_decl(),
            TokenKind::Return => self.parse_return(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::EndOfFile => Ok(AstNode::EndOfFile),
            _ => Err(Error::Parse(ParseError::UnexpectedToken {
                expected: TokenKind::Invalid,
                source_file: self.source_file.clone(),
                found: current_token.clone(),
            })),
        }
    }

    fn parse_function_decl(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Function)?;
        let identifier = self.consume_if(TokenKind::Identifier)?.clone();
        self.consume_if(TokenKind::LParen)?;

        let mut params = vec![];
        if let TokenKind::Identifier = self.current_token().kind {
            params.extend(self.parse_formal_params()?);
        }

        self.consume_if(TokenKind::RParen)?;

        self.consume_if(TokenKind::Arrow)?;

        let return_type = self.consume_if(TokenKind::Type)?.clone();

        // TODO: Add array like function array_list() -> int[] {}
        let block = self.parse_block()?;

        Ok(AstNode::FunctionDecl {
            identifier: identifier.clone(),
            params,
            return_type: return_type.clone(),
            block: Box::new(block),
        })
    }

    fn parse_formal_params(&mut self) -> Result<Vec<AstNode>> {
        let mut params = vec![];

        let first_param = self.parse_formal_param()?;
        params.push(first_param);

        if let TokenKind::Comma = self.current_token().kind {
            self.consume();
            params.extend(self.parse_formal_params()?);
        }

        Ok(params)
    }

    fn parse_formal_param(&mut self) -> Result<AstNode> {
        let identifier = self.consume_if(TokenKind::Identifier)?.clone();
        self.consume_if(TokenKind::Colon)?;
        let param_type = self.consume_if(TokenKind::Type)?.clone();
        Ok(AstNode::FormalParam {
            identifier: identifier.clone(),
            param_type,
        })
    }

    fn parse_while(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::While)?;
        self.consume_if(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.consume_if(TokenKind::RParen)?;
        let block = self.parse_block()?;
        Ok(AstNode::While {
            condition: Box::new(condition),
            body: Box::new(block),
        })
    }

    fn parse_for(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::For)?;
        self.consume_if(TokenKind::LParen)?;

        let initializer = match self.current_token().kind {
            TokenKind::Semicolon => {
                self.consume_if(TokenKind::Semicolon)?;
                None
            }
            TokenKind::Let => Some(self.parse_var_decl()?),
            _ => Err(Error::Parse(ParseError::UnexpectedToken {
                expected: TokenKind::Let,
                found: self.current_token().clone(),
                source_file: self.source_file.clone(),
            }))?,
        };

        let condition = self.parse_expression()?;

        self.consume_if(TokenKind::Semicolon)?;

        let increment = match self.current_token().kind {
            TokenKind::RParen => {
                self.consume_if(TokenKind::RParen)?;
                None
            }
            TokenKind::Identifier => Some(self.parse_assignment_statement()?),
            _ => Err(Error::Parse(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: self.current_token().clone(),
                source_file: self.source_file.clone(),
            }))?,
        };

        match increment {
            Some(_) => Some(self.consume_if(TokenKind::RParen)?),
            None => None,
        };

        let body = self.parse_block()?;

        Ok(AstNode::For {
            initializer: initializer.map(Box::new),
            condition: Box::new(condition),
            increment: increment.map(Box::new),
            body: Box::new(body),
        })
    }

    fn parse_if(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::If)?;
        self.consume_if(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.consume_if(TokenKind::RParen)?;
        let block = self.parse_block()?;

        if self.current_token().kind == TokenKind::Else {
            self.consume();
            let else_block = self.parse_block()?;
            return Ok(AstNode::If {
                condition: Box::new(condition),
                if_true: Box::new(block),
                if_false: Some(Box::new(else_block)),
            });
        }

        Ok(AstNode::If {
            condition: Box::new(condition),
            if_true: Box::new(block),
            if_false: None,
        })
    }

    fn parse_write_box(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadWriteBox)?;
        let loc_x = self.parse_expression()?;
        self.consume_if(TokenKind::Comma)?;
        let loc_y = self.parse_expression()?;
        self.consume_if(TokenKind::Comma)?;
        let width = self.parse_expression()?;
        self.consume_if(TokenKind::Comma)?;
        let height = self.parse_expression()?;
        self.consume_if(TokenKind::Comma)?;
        let colour = self.parse_expression()?;

        self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::PadWriteBox {
            loc_x: Box::new(loc_x),
            loc_y: Box::new(loc_y),
            width: Box::new(width),
            height: Box::new(height),
            colour: Box::new(colour),
        })
    }

    fn parse_write(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadWrite)?;

        let loc_x = self.parse_expression()?;

        self.consume_if(TokenKind::Comma)?;

        let loc_y = self.parse_expression()?;

        self.consume_if(TokenKind::Comma)?;

        let colour = self.parse_expression()?;

        self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::PadWrite {
            loc_x: Box::new(loc_x),
            loc_y: Box::new(loc_y),
            colour: Box::new(colour),
        })
    }

    fn parse_delay(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Delay)?;

        let expr = self.parse_expression()?;

        self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::Delay {
            expression: Box::new(expr),
        })
    }

    fn parse_return(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Return)?;

        let expr = self.parse_expression()?;

        self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::Return {
            expression: Box::new(expr),
        })
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
                    expr: Box::new(left),
                })
            }
            TokenKind::LessThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEqual
            | TokenKind::EqEq
            | TokenKind::NotEqual => {
                self.consume();
                let right = self.parse_expression()?;

                Ok(AstNode::Expression {
                    casted_type: if let TokenKind::As = self.current_token().kind {
                        self.consume();
                        let kind = self.consume_if(TokenKind::Type)?.clone();
                        Some(kind)
                    } else {
                        None
                    },
                    expr: Box::new(AstNode::BinOp {
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
                    right: Box::new(self.parse_simple_expr()?),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_sub_expr(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::LParen)?;
        let expr = self.parse_expression()?;
        self.consume_if(TokenKind::RParen)?;
        Ok(AstNode::SubExpression {
            bin_op: Box::new(expr),
        })
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
                    right: Box::new(self.parse_term()?),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_factor(&mut self) -> Result<AstNode> {
        let next_token = self.current_token().clone();

        match &next_token.kind {
            TokenKind::Identifier => {
                let ident = self.consume_if(TokenKind::Identifier)?.clone();

                if self.current_token().kind == TokenKind::LParen {
                    self.consume();
                    if self.current_token().kind == TokenKind::RParen {
                        self.consume();
                        Ok(AstNode::FunctionCall {
                            identifier: ident.clone(),
                            args: vec![],
                        })
                    } else {
                        let args = self.parse_actual_params()?;
                        self.consume_if(TokenKind::RParen)?;
                        Ok(AstNode::FunctionCall {
                            identifier: ident.clone(),
                            args,
                        })
                    }
                } else {
                    Ok(AstNode::Identifier {
                        token: ident.clone(),
                    })
                }
            }
            TokenKind::PadWidth => self.parse_pad_width(),
            TokenKind::LParen => self.parse_sub_expr(),
            TokenKind::Minus | TokenKind::Not => self.parse_unary_expr(),
            TokenKind::PadRandI => self.parse_pad_rand_i(),
            TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::BoolLiteral
            | TokenKind::ColourLiteral => self.parse_literal(),
            TokenKind::PadHeight => self.parse_pad_height(),
            TokenKind::PadRead => self.parse_pad_read(),
            _ => Err(Error::Parse(ParseError::UnexpectedTokenList {
                expected: vec![TokenKind::Identifier],
                found: next_token.clone(),
                source_file: self.source_file.clone(),
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

    fn parse_pad_height(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadHeight)?;
        Ok(AstNode::PadHeight)
    }

    fn parse_pad_read(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadRead)?;

        let first = self.parse_expression()?;

        self.consume_if(TokenKind::Comma)?;

        let second = self.parse_expression()?;

        Ok(AstNode::PadRead {
            first: Box::new(first),
            second: Box::new(second),
        })
    }

    fn parse_pad_rand_i(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadRandI)?;

        let upper_bound = self.parse_expression()?;

        // self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::PadRandI {
            upper_bound: Box::new(upper_bound),
        })
    }

    // TODO: Add array functionality
    fn parse_var_decl(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Let)?;
        let identifier = self.consume_if(TokenKind::Identifier)?.clone();
        self.consume_if(TokenKind::Colon)?;
        let kind = self.consume_if(TokenKind::Type)?.clone();
        self.consume_if(TokenKind::Equals)?;
        let expression = self.parse_expression()?;
        self.consume_if(TokenKind::Semicolon)?;
        Ok(AstNode::VarDec {
            identifier: identifier.clone(),
            r#type: kind,
            expression: Box::new(expression),
        })
    }

    fn parse_print_statement(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::Print)?;
        let expression = self.parse_expression()?;
        self.consume_if(TokenKind::Semicolon)?;
        Ok(AstNode::Print {
            expression: Box::new(expression),
        })
    }

    fn parse_block(&mut self) -> Result<AstNode> {
        let mut statements = vec![];
        self.consume_if(TokenKind::LBrace)?;
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
                source_file: self.source_file.clone(),
                found: self.current_token().clone(),
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
                source_file: self.source_file.clone(),
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
            TokenKind::IntLiteral => Ok(AstNode::IntLiteral(token)),
            TokenKind::FloatLiteral => Ok(AstNode::FloatLiteral(token)),
            TokenKind::BoolLiteral => Ok(AstNode::BoolLiteral(token)),
            TokenKind::ColourLiteral => Ok(AstNode::ColourLiteral(token)),
            TokenKind::PadHeight => Ok(AstNode::PadHeight),
            TokenKind::PadWidth => Ok(AstNode::PadWidth),
            TokenKind::PadRead => self.parse_pad_read(),
            _ => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Invalid,
                source_file: self.source_file.clone(),
                found: token.clone(),
            }
            .into()),
        }
    }

    fn parse_assignment_statement(&mut self) -> Result<AstNode> {
        let identifier = self.consume_if(TokenKind::Identifier)?.clone();
        self.consume_if(TokenKind::Equals)?;
        let expression = self.parse_expression()?;
        Ok(AstNode::Assignment {
            identifier: identifier.clone(),
            expression: Box::new(expression),
        })
    }

    fn parse_clear_statement(&mut self) -> Result<AstNode> {
        self.consume_if(TokenKind::PadClear)?;
        let expr = self.parse_expression()?;
        self.consume_if(TokenKind::Semicolon)?;

        Ok(AstNode::PadClear {
            expr: Box::new(expr),
        })
    }

    fn parse_actual_params(&mut self) -> Result<Vec<Ast>> {
        let mut params = vec![Box::from(self.parse_expression()?)];

        if let TokenKind::Comma = self.current_token().kind {
            self.consume();

            match self.current_token().kind {
                TokenKind::RParen => return Ok(params),
                _ => params.extend(self.parse_actual_params()?),
            }
        }

        Ok(params)
    }
}
