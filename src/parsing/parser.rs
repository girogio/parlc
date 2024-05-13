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

    fn current_token_kind(&self) -> &TokenKind {
        match self.tokens.get(self.current) {
            Some(k) => &k.kind,
            None => &TokenKind::EndOfFile,
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
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

    fn assert_token_is_any<T: Clone + IntoIterator<Item = TokenKind>>(
        &self,
        possible_kinds: T,
    ) -> Result<()> {
        if !possible_kinds
            .clone()
            .into_iter()
            .any(|kind| self.current_token().kind == kind)
        {
            Err(ParseError::UnexpectedTokenList {
                source_file: self.source_file.clone(),
                found: self.current_token(),
                expected: possible_kinds.into_iter().collect(),
            }
            .into())
        } else {
            Ok(())
        }
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
        self.assert_token_is_any([
            TokenKind::Let,
            TokenKind::Print,
            TokenKind::Delay,
            TokenKind::PadWrite,
            TokenKind::PadWriteBox,
            TokenKind::PadClear,
            TokenKind::If,
            TokenKind::Identifier,
            TokenKind::For,
            TokenKind::While,
            TokenKind::Function,
            TokenKind::Return,
            TokenKind::LBrace,
            TokenKind::EndOfFile,
        ])?;

        match self.current_token_kind() {
            TokenKind::Let => self.parse_var_decl(),
            TokenKind::Identifier => match self.peek_token() {
                Some(tok) => match tok.kind {
                    TokenKind::LBracket | TokenKind::Equals => {
                        let assignment_stmnt = self.parse_assignment_statement();
                        self.consume_if(TokenKind::Semicolon)?;
                        assignment_stmnt
                    }
                    _ => self.parse_identifier(),
                },
                None => self.parse_identifier(),
            },
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
                found: self.current_token(),
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

        let index = if let TokenKind::LBracket = self.current_token().kind {
            self.consume_if(TokenKind::LBracket)?;

            let index = self.consume_if(TokenKind::IntLiteral)?.clone();

            self.consume_if(TokenKind::RBracket)?;

            Some(index)
        } else {
            None
        };

        Ok(AstNode::FormalParam {
            identifier: identifier.clone(),
            param_type,
            index,
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
        let left = self.parse_equality()?;

        match self.current_token().kind {
            TokenKind::As => {
                self.consume();
                let kind = self.consume_if(TokenKind::Type)?.clone();
                Ok(AstNode::Expression {
                    casted_type: Some(kind),
                    expr: Box::new(left),
                })
            }

            TokenKind::And | TokenKind::Or => {
                let operator = self.consume().clone();
                let right = self.parse_expression()?;

                let bin_op = AstNode::BinOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };

                Ok(AstNode::Expression {
                    casted_type: None,
                    expr: Box::new(bin_op),
                })
            }

            _ => Ok(AstNode::Expression {
                casted_type: None,
                expr: Box::new(left),
            }),
        }
    }

    fn parse_equality(&mut self) -> Result<AstNode> {
        let left = self.parse_comparison()?;
        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::EqEq | TokenKind::NotEqual => {
                self.consume();
                let right = self.parse_comparison()?;
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator: curr,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_comparison(&mut self) -> Result<AstNode> {
        let left = self.parse_term()?;
        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::LessThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEqual => {
                self.consume();
                let right = self.parse_term()?;
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator: curr,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_term(&mut self) -> Result<AstNode> {
        let left = self.parse_factor()?;
        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::Plus | TokenKind::Minus => {
                let operator = self.consume().clone();
                let right = self.parse_term()?;
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_factor(&mut self) -> Result<AstNode> {
        let left = self.parse_unary()?;
        let curr = self.current_token().clone();

        match &curr.kind {
            TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod => {
                let operator = self.consume().clone();
                let right = self.parse_factor()?;
                Ok(AstNode::BinOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_unary(&mut self) -> Result<AstNode> {
        let curr_token = self.current_token();

        match curr_token.kind {
            TokenKind::Minus | TokenKind::Not => {
                self.consume();
                let expr = self.parse_primary()?;
                Ok(AstNode::UnaryOp {
                    operator: curr_token.clone(),
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<AstNode> {
        self.assert_token_is_any([
            TokenKind::Identifier,
            TokenKind::IntLiteral,
            TokenKind::FloatLiteral,
            TokenKind::BoolLiteral,
            TokenKind::ColourLiteral,
            TokenKind::PadHeight,
            TokenKind::PadWidth,
            TokenKind::PadRead,
            TokenKind::PadRandI,
            TokenKind::LParen,
        ])?;

        match self.current_token_kind() {
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
                } else if let TokenKind::LBracket = self.current_token_kind() {
                    self.consume();
                    let index = self.parse_expression()?;
                    self.consume_if(TokenKind::RBracket)?;

                    return Ok(AstNode::ArrayAccess {
                        identifier: ident.clone(),
                        index: Box::new(index),
                    });
                } else {
                    Ok(AstNode::Identifier {
                        token: ident.clone(),
                    })
                }
            }

            TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::BoolLiteral
            | TokenKind::ColourLiteral => self.parse_literal(),
            TokenKind::PadHeight => self.parse_pad_height(),
            TokenKind::PadWidth => self.parse_pad_width(),
            TokenKind::PadRead => self.parse_pad_read(),
            TokenKind::PadRandI => self.parse_pad_rand_i(),
            TokenKind::LParen => self.parse_sub_expr(),
            _ => unreachable!(),
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

    fn parse_identifier(&mut self) -> Result<AstNode> {
        let ident = self.consume_if(TokenKind::Identifier)?.clone();

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
            x: Box::new(first),
            y: Box::new(second),
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
        let element_type = self.consume_if(TokenKind::Type)?.clone();

        self.assert_token_is_any([TokenKind::Equals, TokenKind::LBracket])?;

        match self.current_token_kind() {
            TokenKind::Equals => {
                self.consume();
                let expression = self.parse_expression()?;
                self.consume_if(TokenKind::Semicolon)?;
                Ok(AstNode::VarDec {
                    identifier: identifier.clone(),
                    r#type: element_type,
                    expression: Box::new(expression),
                })
            }
            TokenKind::LBracket => {
                self.consume();
                self.consume_if(TokenKind::RBracket)?;
                self.consume_if(TokenKind::Equals)?;
                self.consume_if(TokenKind::LBracket)?;
                let elements = self.parse_array_elements()?;
                let size = elements.len();
                self.consume_if(TokenKind::RBracket)?;
                self.consume_if(TokenKind::Semicolon)?;
                Ok(AstNode::VarDecArray {
                    identifier,
                    element_type,
                    size,
                    elements,
                })
            }
            _ => Err(ParseError::UnexpectedTokenList {
                expected: vec![TokenKind::Equals, TokenKind::LBracket],
                source_file: self.source_file.clone(),
                found: self.current_token().clone(),
            }
            .into()),
        }
    }

    fn parse_array_elements(&mut self) -> Result<Vec<AstNode>> {
        let mut elements = vec![];
        let first_elem = self.parse_literal()?;
        elements.push(first_elem);

        if let TokenKind::Comma = self.current_token().kind {
            self.consume();
            elements.extend(self.parse_array_elements()?);
        }

        Ok(elements)
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

        if let TokenKind::LBracket = self.current_token_kind() {
            self.consume();
            let index = self.parse_expression()?;
            self.consume_if(TokenKind::RBracket)?;
            self.consume_if(TokenKind::Equals)?;
            let expression = self.parse_expression()?;
            Ok(AstNode::Assignment {
                identifier: identifier.clone(),
                index: Some(Box::new(index)),
                expression: Box::new(expression),
            })
        } else {
            self.consume_if(TokenKind::Equals)?;
            let expression = self.parse_expression()?;
            Ok(AstNode::Assignment {
                identifier: identifier.clone(),
                index: None,
                expression: Box::new(expression),
            })
        }
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
