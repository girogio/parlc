use crate::{
    errors::{LexicalError, LexicalErrorKind, Result},
    scanner::token::{TextSpan, Token, TokenKind},
    utils::{Dfsa, Stream},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Other,
    Whitespace,
    Register,
    Digit,
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'r' => Category::Register,
            '0'..='9' => Category::Digit,
            ' ' | '\n' | '\t' => Category::Whitespace,
            _ => Category::Other,
        }
    }
}

const ERR_STATE: i32 = -2;
const BAD_STATE: i32 = -1;

pub struct Lexer<B: Stream> {
    buffer: B,
    dfsa: Dfsa<i32, Category, fn(i32, Category) -> i32>,
}
impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        match i {
            2 => TokenKind::Register,
            3 => TokenKind::Whitespace,
            _ => TokenKind::Invalid,
        }
    }
}

impl<B: Stream> Lexer<B> {
    pub fn new(input: &str) -> Self {
        Lexer {
            buffer: B::new(input),
            dfsa: Dfsa::new(vec![2, 3], 0, |state, category| match (state, category) {
                (0, Category::Register) => 1,
                (0, Category::Whitespace) => 3,
                (1, Category::Digit) => 2,
                (2, Category::Digit) => 2,
                (3, Category::Whitespace) => 3,
                (3, _) => 0,
                _ => ERR_STATE,
            }),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let mut state = self.dfsa.start_state();
        let mut lexeme = String::new();
        let mut stack = vec![BAD_STATE];
        let start = self.buffer.get_input_pointer();

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

        let text_span = TextSpan::new(start, self.buffer.get_input_pointer(), &lexeme);

        match self.dfsa.is_accepting(&state) {
            true => Ok(Token::new(TokenKind::from(state), text_span)),
            false => Err(LexicalError::new(LexicalErrorKind::InvalidCharacter, text_span).into()),
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
                    TextSpan::new(
                        self.buffer.get_input_pointer(),
                        self.buffer.get_input_pointer(),
                        "",
                    ),
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
