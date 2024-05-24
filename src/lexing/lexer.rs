use std::path::Path;

use crate::{
    core::{TextSpan, Token, TokenKind},
    utils::{
        errors::{Error, LexicalError},
        Stream,
    },
};

use super::dfsa::{Category, Dfsa, DfsaBuilder};

pub struct Lexer<B: Stream> {
    buffer: B,
    dfsa: Dfsa,
}

impl<B: Stream + Clone> Lexer<B> {
    pub fn new(input: &str, file: &Path, dfsa: Option<Dfsa>) -> Self {
        let mut dfsa_builder = DfsaBuilder::new();

        match dfsa {
            Some(dfsa) => Lexer {
                buffer: B::new(input, file),
                dfsa,
            },
            None => {
                let dfsa = dfsa_builder
                    .add_category('a'..='f', Category::HexAndLetter)
                    .add_category('A'..='F', Category::HexAndLetter)
                    .add_category('g'..='z', Category::Letter)
                    .add_category('G'..='Z', Category::Letter)
                    .add_category('0'..='9', Category::Digit)
                    .add_multiple_single_final_character_symbols(vec![
                        ('\n', Category::Newline, TokenKind::Newline),
                        ('{', Category::LBrace, TokenKind::LBrace),
                        ('}', Category::RBrace, TokenKind::RBrace),
                        ('(', Category::LParen, TokenKind::LParen),
                        (')', Category::RParen, TokenKind::RParen),
                        ('[', Category::LBracket, TokenKind::LBracket),
                        (']', Category::RBracket, TokenKind::RBracket),
                        (';', Category::Semicolon, TokenKind::Semicolon),
                        (':', Category::Colon, TokenKind::Colon),
                        ('+', Category::Plus, TokenKind::Plus),
                        ('*', Category::Asterisk, TokenKind::Multiply),
                        (',', Category::Comma, TokenKind::Comma),
                        ('\0', Category::Eof, TokenKind::EndOfFile),
                        ('%', Category::Percent, TokenKind::Mod),
                    ])
                    .add_whitespace_logic()
                    .add_comment_functionality()
                    .add_multi_char_rel_ops()
                    .add_identifier_logic()
                    .add_number_logic()
                    .build();

                Lexer {
                    buffer: B::new(input, file),
                    dfsa,
                }
            }
        }
    }

    /// These are the reserved keywords in the language. Note that these must be
    /// valid identifiers, otherwise they won't be caught.
    fn handle_keyword(&self, lexeme: &str) -> TokenKind {
        match lexeme {
            "__delay" => TokenKind::Delay,
            "__height" => TokenKind::PadHeight,
            "__print" => TokenKind::Print,
            "__randi" => TokenKind::PadRandI,
            "__read" => TokenKind::PadRead,
            "__width" => TokenKind::PadWidth,
            "__clear" => TokenKind::PadClear,
            "__write_box" => TokenKind::PadWriteBox,
            "__write" => TokenKind::PadWrite,
            "and" => TokenKind::And,
            "as" => TokenKind::As,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "fun" => TokenKind::Function,
            "if" => TokenKind::If,
            "int" | "float" | "bool" | "colour" => TokenKind::Type,
            "let" => TokenKind::Let,
            "not" => TokenKind::Not,
            "or" => TokenKind::Or,
            "return" => TokenKind::Return,
            "true" | "false" => TokenKind::BoolLiteral,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        let mut state = self.dfsa.start_state();
        let mut lexeme = String::new();
        let mut stack = vec![self.dfsa.bad_state()];
        let mut prev_state = state;

        let (start_line, start_col) = (self.buffer.get_line(), self.buffer.get_col());

        while state != self.dfsa.error_state() {
            prev_state = state;
            let c = self.buffer.next_char();

            lexeme.push(c);

            if self.dfsa.is_accepting(&state) {
                stack = vec![self.dfsa.bad_state()];
            }

            stack.push(state);

            let cat = self.dfsa.get_category(c);
            state = self.dfsa.delta(state, cat);
        }

        while !self.dfsa.is_accepting(&state) && state != self.dfsa.bad_state() {
            state = stack.pop().unwrap();
            if state != self.dfsa.bad_state() {
                lexeme.pop();
                self.buffer.rollback();
            }
        }

        let (end_line, end_col) = (self.buffer.get_line(), self.buffer.get_col());

        let text_span = TextSpan::new(start_line, end_line, start_col, end_col, &lexeme);

        match self.dfsa.is_accepting(&state) {
            true => Ok(Token::new(
                match self.dfsa.get_token_kind(state) {
                    TokenKind::Identifier => self.handle_keyword(&lexeme),
                    _ => self.dfsa.get_token_kind(state),
                },
                text_span,
            )),
            false => {
                let error = match prev_state {
                    _ if self.dfsa.is_accepting(&prev_state) => LexicalError::InvalidCharacter(
                        TextSpan::new(start_line, start_line, start_col, start_col, &lexeme),
                    ),
                    _ => LexicalError::InvalidCharacter(TextSpan::new(
                        start_line, end_line, start_col, end_col, &lexeme,
                    )),
                };
                self.buffer.next_char();
                Err(Error::Lexical(error))
            }
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

#[cfg(test)]
mod tests {
    use crate::utils::SimpleBuffer;

    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;

    #[rstest]
    fn test_lex() {
        let input = "fn( bruh ) { return test; }";
        let fake_path = Path::new("fake_path");
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input, fake_path, None);
        let tokens = lexer.lex();

        assert_matches!(tokens, Ok(tokens) => tokens);
    }
}
