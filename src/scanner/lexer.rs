use crate::{
    core::TokenKind,
    models::{TextSpan, Token},
    utils::{
        errors::{Error, LexicalError},
        Stream,
    },
};

use super::dfsa::Dfsa;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Whitespace,
    Letter,
    Digit,
    Underscore,
    Newline,
    RBrace,
    LBrace,
    RParen,
    LParen,
    RBracket,
    LBracket,
    DoubleQuote,
    SingleQuote,
    Semicolon,
    Colon,
    Backslash,
    Eof,
    Other,
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'a'..='z' | 'A'..='Z' => Category::Letter,
            '0'..='9' => Category::Digit,
            '_' => Category::Underscore,
            ' ' | '\t' => Category::Whitespace,
            '\n' => Category::Newline,
            '{' => Category::LBrace,
            '}' => Category::RBrace,
            '(' => Category::LParen,
            ')' => Category::RParen,
            ':' => Category::Colon,
            '\\' => Category::Backslash,
            '[' => Category::LBracket,
            ']' => Category::RBracket,
            '"' => Category::DoubleQuote,
            '\'' => Category::SingleQuote,
            ';' => Category::Semicolon,
            '\0' => Category::Eof,
            _ => Category::Other,
        }
    }
}

impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        match i {
            10 => TokenKind::Whitespace,
            20 => TokenKind::Newline,
            30 => TokenKind::Identifier(String::new()),
            40 => TokenKind::LBrace,
            50 => TokenKind::RBrace,
            60 => TokenKind::LParen,
            70 => TokenKind::RParen,
            80 => TokenKind::LBracket,
            90 => TokenKind::RBracket,
            101 => TokenKind::StringLiteral(String::new()),
            110 => TokenKind::SingleQuote,
            120 => TokenKind::Semicolon,
            130 => TokenKind::Colon,
            _ => TokenKind::Invalid,
        }
    }
}
pub struct Lexer<B: Stream> {
    buffer: B,
    dfsa: Dfsa<i32, Category, fn(i32, Category) -> i32>,
}

const ERR_STATE: i32 = -2;
const BAD_STATE: i32 = -1;

impl<B: Stream> Lexer<B> {
    pub fn new(input: &str) -> Self {
        Lexer {
            buffer: B::new(input),
            dfsa: Dfsa::new(
                vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 101, 110, 120, 130],
                0,
                |state, category| match (state, category) {
                    (0, Category::Whitespace) => 10,
                    (0, Category::Newline) => 20,
                    // Single character tokens
                    (0, Category::Letter) => 30,
                    (0, Category::LBrace) => 40,
                    (0, Category::RBrace) => 50,
                    (0, Category::LParen) => 60,
                    (0, Category::RParen) => 70,
                    (0, Category::LBracket) => 80,
                    (0, Category::RBracket) => 90,
                    (0, Category::SingleQuote) => 110,
                    (0, Category::Semicolon) => 120,
                    (0, Category::Colon) => 130,
                    (10, Category::Whitespace) => 10,
                    // String literal logic
                    (0, Category::DoubleQuote) => 100,
                    (100, Category::DoubleQuote) => 101,
                    (100, Category::Backslash) => 102,
                    (102, Category::DoubleQuote) => 100,
                    (100, Category::Eof) => ERR_STATE,
                    (100, _) => 100,
                    // Identifier logic
                    (30, Category::Letter) => 30,
                    (30, Category::Digit) => 30,
                    (30, Category::Underscore) => 30,
                    // Map all other characters to the error state
                    _ => ERR_STATE,
                },
            ),
        }
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        let mut state = *self.dfsa.start_state();
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

            let cat = Category::from(c);
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
                match TokenKind::from(state) {
                    TokenKind::StringLiteral(_) => TokenKind::StringLiteral(lexeme),
                    TokenKind::Identifier(_) => TokenKind::Identifier(lexeme),
                    _ => TokenKind::from(state),
                },
                text_span,
            )),
            false => {
                let error = match prev_state {
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

    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            let token = self.next_token();

            match token {
                Ok(token) => tokens.push(token),
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

                if errors.is_empty() {
                    return Ok(tokens);
                } else {
                    return Err(errors);
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
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input);
        let tokens = lexer.lex();

        assert_matches!(tokens, Ok(tokens) => tokens);
    }
}
