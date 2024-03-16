use crate::{
    core::{DataTypes, TextSpan, Token, TokenKind},
    utils::{
        errors::{Error, LexicalError},
        Stream,
    },
};

use super::dfsa::{Category, Dfsa, DfsaBuilder};

pub struct Lexer<B: Stream + Clone> {
    buffer: B,
    dfsa: Dfsa,
}

const ERR_STATE: i32 = -2;
const BAD_STATE: i32 = -1;

impl<B: Stream + Clone> Lexer<B> {
    pub fn new(input: &str) -> Self {
        let dfsa_builder = DfsaBuilder::new();

        let dfsa = dfsa_builder
            .add_repeatable_final_character_symbol(' ', Category::Whitespace, TokenKind::Whitespace)
            .add_category(['/'], Category::Slash)
            .add_category(['.'], Category::Period)
            .add_category(['_'], Category::Underscore)
            .add_category(['\''], Category::SingleQuote)
            .add_category(['"'], Category::DoubleQuote)
            .add_single_final_character_symbol('\n', Category::Newline, TokenKind::Newline)
            .add_single_final_character_symbol('{', Category::LBrace, TokenKind::LBrace)
            .add_single_final_character_symbol('}', Category::RBrace, TokenKind::RBrace)
            .add_single_final_character_symbol('(', Category::LParen, TokenKind::LParen)
            .add_single_final_character_symbol(')', Category::RParen, TokenKind::RParen)
            .add_single_final_character_symbol('[', Category::LBracket, TokenKind::LBracket)
            .add_single_final_character_symbol(']', Category::RBracket, TokenKind::RBracket)
            .add_single_final_character_symbol(';', Category::Semicolon, TokenKind::Semicolon)
            .add_single_final_character_symbol(':', Category::Colon, TokenKind::Colon)
            .add_single_final_character_symbol('=', Category::Equals, TokenKind::Equals)
            .add_single_final_character_symbol('+', Category::Plus, TokenKind::Plus)
            .add_single_final_character_symbol('-', Category::Minus, TokenKind::Minus)
            .add_single_final_character_symbol('*', Category::Asterisk, TokenKind::Asterisk)
            .add_single_final_character_symbol('\0', Category::Eof, TokenKind::EndOfFile)
            .build();
        

        // let literal_state = lexer_builder
        //     .auto_add_transition(0, [Category::Letter, Category::Underscore])
        //     .add_transition(literal_state, [Category::Letter], literal_state);

        Lexer {
            buffer: B::new(input),
            dfsa,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        let mut state = self.dfsa.start_state();
        let mut lexeme = String::new();
        let mut stack = vec![BAD_STATE];
        let mut prev_state = state;
        let (start_line, start_col) = (self.buffer.get_line(), self.buffer.get_col());

        while state != ERR_STATE {
            prev_state = state;
            let c = self.buffer.next_char();

            lexeme.push(c);

            if state == 100 && c == '\\' {
                lexeme.pop();
            }

            if self.dfsa.is_accepting(&state) {
                stack = vec![BAD_STATE];
            }

            stack.push(state);

            let cat = self.dfsa.get_category(c);
            state = self.dfsa.delta(state, cat);

            // If we just starded parsing a string literal
            if (prev_state, state) == (0, 100) {
                lexeme.pop();
            }

            if (prev_state, state) == (100, 101) {
                lexeme.pop();
            }
        }

        while !self.dfsa.is_accepting(&state) && state != BAD_STATE {
            state = stack.pop().unwrap();
            if state != BAD_STATE {
                lexeme.pop();
                self.buffer.rollback();
            }
        }

        let (end_line, end_col) = (self.buffer.get_line(), self.buffer.get_col());

        let text_span = TextSpan::new(start_line, end_line, start_col, end_col, &lexeme);

        match self.dfsa.is_accepting(&state) {
            true => Ok(Token::new(
                match self.dfsa.get_token_kind(state) {
                    TokenKind::FloatLiteral(_) => TokenKind::FloatLiteral(lexeme),
                    TokenKind::IntLiteral(_) => TokenKind::IntLiteral(lexeme.parse().unwrap()),
                    TokenKind::StringLiteral(_) => TokenKind::StringLiteral(lexeme),
                    TokenKind::Identifier(_) => self.handle_keyword(&lexeme),
                    _ => self.dfsa.get_token_kind(state),
                },
                text_span,
            )),
            false => {
                let error = match prev_state {
                    35 => LexicalError::UnterminatedBlockComment(text_span),
                    150 => LexicalError::InvalidFloatLiteral(TextSpan::new(
                        start_line,
                        end_line,
                        start_col,
                        end_col,
                        &self.buffer.current_char().to_string(),
                    )),
                    100 => LexicalError::UnterminatedString(text_span),
                    _ => LexicalError::InvalidCharacter(TextSpan::new(
                        start_line,
                        end_line,
                        start_col,
                        end_col,
                        &self.buffer.current_char().to_string(),
                    )),
                };
                self.buffer.next_char();
                Err(error.into())
            }
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, Error> {
        let buffer = self.buffer.clone();
        let token = self.next_token();
        self.buffer = buffer;
        token
    }

    fn handle_keyword(&self, lexeme: &str) -> TokenKind {
        match lexeme {
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "fn" => TokenKind::Function,
            "else" => TokenKind::Else,
            "let" => TokenKind::Let,
            "while" => TokenKind::While,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "int" => TokenKind::Type(DataTypes::Int),
            "float" => TokenKind::Type(DataTypes::Float),
            "true" => TokenKind::BoolLiteral(true),
            "false" => TokenKind::BoolLiteral(false),
            "bool" => TokenKind::Type(DataTypes::Bool),
            "colour" => TokenKind::Type(DataTypes::Colour),
            "__width" => TokenKind::PadWidth,
            "__height" => TokenKind::PadHeight,
            "__read" => TokenKind::PadRead,
            "__print" => TokenKind::Print,
            "__randi" => TokenKind::PadRandI,
            _ => TokenKind::Identifier(lexeme.to_string()),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            let token = self.next_token();

            match token {
                Ok(token) => {
                    if token.kind != TokenKind::Whitespace
                        && token.kind != TokenKind::Comment
                        && token.kind != TokenKind::Newline
                    {
                        tokens.push(token);
                    }
                }
                Err(err) => errors.push(err),
            }

            if self.buffer.is_eof() {
                tokens.push(Token::new(
                    TokenKind::EndOfFile,
                    TextSpan::new(
                        self.buffer.get_line(),
                        self.buffer.get_line(),
                        self.buffer.get_col(),
                        self.buffer.get_col(),
                        "\0",
                    ),
                ));

                match errors.is_empty() {
                    true => return Ok(tokens),
                    false => return Err(errors),
                }
            }
        }
    }
}

// example usage

#[cfg(test)]
mod tests {
    use crate::utils::SimpleBuffer;

    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;

    #[rstest]
    fn test_lex() {
        let input = "fn( bruh ) { return test; }";
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input);
        let tokens = lexer.lex();

        assert_matches!(tokens, Ok(tokens) => tokens);
    }
}
