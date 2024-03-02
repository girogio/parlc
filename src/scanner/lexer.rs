use crate::{
    core::{TextSpan, Token, TokenKind},
    errors::{LexicalError, LexicalErrorKind, Result},
    scanner::Dfsa,
    utils::Stream,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Other,
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
            '[' => Category::LBracket,
            ']' => Category::RBracket,
            '"' => Category::DoubleQuote,
            '\'' => Category::SingleQuote,
            ';' => Category::Semicolon,
            _ => Category::Other,
        }
    }
}

pub struct Lexer<B: Stream> {
    buffer: B,
    dfsa: Dfsa<i32, Category, fn(i32, Category) -> i32>,
}

impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        match i {
            1 => TokenKind::Whitespace,
            2 => TokenKind::Newline,
            3 => TokenKind::Identifier,
            4 => TokenKind::LBrace,
            5 => TokenKind::RBrace,
            6 => TokenKind::LParen,
            7 => TokenKind::RParen,
            8 => TokenKind::LBracket,
            9 => TokenKind::RBracket,
            10 => TokenKind::DoubleQuote,
            11 => TokenKind::SingleQuote,
            12 => TokenKind::Semicolon,
            _ => TokenKind::Invalid,
        }
    }
}

const ERR_STATE: i32 = -2;
const BAD_STATE: i32 = -1;

impl<B: Stream> Lexer<B> {
    pub fn new(input: &str) -> Self {
        Lexer {
            buffer: B::new(input),
            dfsa: Dfsa::new(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                0,
                |state, category| match (state, category) {
                    (0, Category::Whitespace) => 1,
                    (0, Category::Newline) => 2,
                    (0, Category::Letter) => 3,
                    (0, Category::LBrace) => 4,
                    (0, Category::RBrace) => 5,
                    (0, Category::LParen) => 6,
                    (0, Category::RParen) => 7,
                    (0, Category::LBracket) => 8,
                    (0, Category::RBracket) => 9,
                    (0, Category::DoubleQuote) => 10,
                    (0, Category::SingleQuote) => 11,
                    (0, Category::Semicolon) => 12,
                    (1, Category::Whitespace) => 1,
                    (3, Category::Letter) => 3,
                    (3, Category::Digit) => 3,
                    (3, Category::Underscore) => 3,

                    _ => ERR_STATE,
                },
            ),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let mut state = self.dfsa.start_state();
        let mut lexeme = String::new();
        let mut stack = vec![BAD_STATE];

        while state != ERR_STATE {
            let c = self.buffer.next_char();

            lexeme += &c.to_string();

            if self.dfsa.is_accepting(&state) {
                stack = vec![BAD_STATE];
            }

            stack.push(state);

            let cat = Category::from(c);
            state = self.dfsa.delta(state, cat);
        }

        while !self.dfsa.is_accepting(&state) && state != BAD_STATE {
            state = stack.pop().unwrap();
            if state != BAD_STATE {
                lexeme.pop();
                self.buffer.rollback();
            }
        }

        let text_span = TextSpan::new(self.buffer.get_line(), self.buffer.get_col(), &lexeme);

        match self.dfsa.is_accepting(&state) {
            true => Ok(Token::new(TokenKind::from(state), text_span)),
            false => Err({
                println!("{state}");
                LexicalError::new(LexicalErrorKind::InvalidCharacter, text_span).into()
            }),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    tokens.push(token);
                }
                Err(e) => {
                    return Err(e);
                }
            }
            if self.buffer.is_eof() {
                tokens.push(Token::new(
                    TokenKind::EndOfFile,
                    TextSpan::new(self.buffer.get_line(), self.buffer.get_col(), ""),
                ));

                return Ok(tokens);
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
        let input = "r0123 r2456 \n r1234";
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input);
        let tokens = lexer.lex();

        let tokens = assert_matches!(tokens, Ok(tokens) => tokens);

        for token in &tokens {
            println!("{}", token);
        }

        assert_eq!(tokens.len(), 6);
    }
}
