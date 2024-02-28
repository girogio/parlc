use crate::ast::token::{TextSpan, Token, TokenKind};
/// Why is the lifetime parameter 'a necessary?
///
/// The lifetime parameter 'a is necessary because the Lexer struct contains a
/// reference to the input string. The lifetime parameter 'a ensures that the
/// input string outlives the Lexer instance.
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len() {
            return None;
        }

        if self.current_pos == self.input.len() {
            let eof_char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, &eof_char.to_string()),
            ));
        }

        let c = self.current_char();

        c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::BadToken;

            if Self::is_number_start(c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::IntegerLiteral(number);
            } else if Self::is_additive_op_start(c) {
                kind = self.consume_additive_op();
            } else if c.is_whitespace() {
                self.consume();
                kind = TokenKind::Whitespace
            } else {
                self.consume();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, &literal);
            Token::new(kind, span)
        })
    }

    /// This is allowed to panic since we are accounting for the possibility of
    /// the input string being empty in the next_token method.
    fn current_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    fn peek(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }

    fn is_number_start(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_additive_op_start(c: char) -> bool {
        c == '+' || c == '-' || c == 'a'
    }

    fn consume_additive_op(&mut self) -> TokenKind {
        let mut kind = TokenKind::BadToken;

        while let Some(c) = self.current_char() {
            if c == '+' {
                self.consume().unwrap();
                kind = TokenKind::AddOp;
            } else if c == '-' {
                self.consume().unwrap();
                kind = TokenKind::AddOp;
            } else if c == 'a' {
                while let Some(c) = self.consume() {
                    if c == 'n' {
                        if let Some(c) = self.consume() {
                            if c == 'd' {
                                kind = TokenKind::AddOp;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        kind
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("2 3 4");
        let mut tokens = vec![];

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        // The input string "2 3 4" contains 3 numbers and 2 spaces, and one EOF
        assert_eq!(tokens.len(), 5 + 1);
    }
}
