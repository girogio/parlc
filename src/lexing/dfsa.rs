use std::{cmp::max, collections::HashMap};

use crate::core::TokenKind;

use super::transition::Transition;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
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
    Period,
    Semicolon,
    Asterisk,
    Colon,
    Equals,
    Slash,
    Plus,
    LessThan,
    GreaterThan,
    Minus,
    Eof,
    Other,
    Comma,
    Hashtag,
    Any,
    Exclamation,
    HexAndLetter,
}

#[derive(Debug)]
pub struct Dfsa {
    accepted_states: Vec<i32>,
    character_table: HashMap<char, Category>,
    transition_table: HashMap<(i32, Category), i32>,
    state_to_token: HashMap<i32, TokenKind>,
}

impl Dfsa {
    fn new(
        accepted_states: Vec<i32>,
        character_table: HashMap<char, Category>,
        transition_table: HashMap<(i32, Category), i32>,
        state_to_token: HashMap<i32, TokenKind>,
    ) -> Self {
        Dfsa {
            accepted_states,
            character_table,
            transition_table,
            state_to_token,
        }
    }

    pub fn start_state(&self) -> i32 {
        0
    }

    pub fn error_state(&self) -> i32 {
        -2
    }

    pub fn bad_state(&self) -> i32 {
        -1
    }

    pub fn get_token_kind(&self, state: i32) -> TokenKind {
        *self
            .state_to_token
            .get(&state)
            .unwrap_or(&TokenKind::Invalid)
    }

    pub fn is_accepting(&self, state: &i32) -> bool {
        self.accepted_states.contains(state)
    }

    pub fn get_category(&self, c: char) -> Category {
        *self.character_table.get(&c).unwrap_or(&Category::Other)
    }

    pub fn delta(&self, state: i32, category: Category) -> i32 {
        let next_state = self.transition_table.get(&(state, category));

        match next_state {
            Some(next_state) => *next_state,
            None => match self.transition_table.get(&(state, Category::Any)) {
                Some(next_state) => *next_state,
                None => -2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DfsaBuilder {
    pub max_state: i32,
    pub accepted_states: Vec<i32>,
    pub character_table: HashMap<char, Category>,
    pub transition_table: HashMap<(i32, Category), i32>,
    pub state_to_token: HashMap<i32, TokenKind>,
}

impl DfsaBuilder {
    /// Creates a new, empty DfsaBuilder
    pub fn new() -> Self {
        DfsaBuilder {
            max_state: 0,
            accepted_states: vec![],
            character_table: HashMap::new(),
            transition_table: HashMap::new(),
            state_to_token: HashMap::new(),
        }
    }

    /// Add a range of characters to the character table, mapping to the same
    /// category
    ///
    /// Example: adding all letters to the character table
    pub fn add_category<T: IntoIterator<Item = char>>(
        &mut self,
        range: T,
        category: Category,
    ) -> &mut Self {
        for c in range {
            self.character_table.insert(c, category);
        }

        self
    }

    pub fn add_transition<T: IntoIterator<Item = Category>>(
        &mut self,
        state: i32,
        category: T,
        next_state: i32,
    ) -> &mut Self {
        category.into_iter().for_each(|cat| {
            self.transition_table.insert((state, cat), next_state);
        });
        self
    }

    pub fn auto_add_transition(
        &mut self,
        state: i32,
        category: Category,
        to: Option<i32>,
        token_kind: Option<TokenKind>,
    ) -> i32 {
        let next_state = to.unwrap_or(self.max_state + 1);
        self.transition_table.insert((state, category), next_state);
        if let Some(token_kind) = token_kind {
            self.accepted_states.push(next_state);
            self.state_to_token.insert(next_state, token_kind);
        }
        self.max_state = max(self.max_state, next_state);
        next_state
    }

    pub fn transition(&mut self) -> Transition {
        self.max_state += 1;
        Transition::new(self)
    }

    pub fn add_single_final_character_symbol(
        &mut self,
        character: char,
        category: Category,
        token_kind: TokenKind,
    ) -> &mut Self {
        self.max_state += 1;
        self.character_table.insert(character, category);
        self.accepted_states.push(self.max_state);
        self.state_to_token.insert(self.max_state, token_kind);
        self.add_transition(0, [category], self.max_state);

        self
    }

    pub fn add_multiple_single_final_character_symbols(
        &mut self,
        things_to_add: Vec<(char, Category, TokenKind)>,
    ) -> &mut Self {
        for (character, category, token_kind) in things_to_add {
            self.add_single_final_character_symbol(character, category, token_kind);
        }

        self
    }

    pub fn add_comment_functionality(&mut self) -> &mut Self {
        self.add_single_final_character_symbol('\n', Category::Newline, TokenKind::Newline)
            // Add required characters for comment lexing
            .add_category(['\0'], Category::Eof)
            .add_category(['/'], Category::Slash)
            .add_category(['*'], Category::Asterisk)
            .transition() // If just a '/' is present, then we can say that's a division operator
            .to([Category::Slash])
            .goes_to(TokenKind::Divide)
            .done();

        let slash_state = self.max_state; // Get the state id of the slash state

        let in_multiline_comment_state = // If we see an asterisk, we can assume that we are now inside a multiline comment
            self.auto_add_transition(slash_state, Category::Asterisk, None, None); // Still not final as we need it to be closed

        // If we're at the end of a file and the comment is not closed, then it's an error
        self.auto_add_transition(in_multiline_comment_state, Category::Eof, Some(-2), None);

        // Catch any other character and stay in the multiline comment state
        self.auto_add_transition(
            in_multiline_comment_state,
            Category::Any,
            Some(in_multiline_comment_state),
            None,
        );

        // Unless it's a '*' , which means we're about to close the comment.
        let multiline_comment_end_asterisk_state =
            self.auto_add_transition(in_multiline_comment_state, Category::Asterisk, None, None);

        // Cloes the multiline comment with a slash after the previous '*', and return a final state
        self.auto_add_transition(
            multiline_comment_end_asterisk_state,
            Category::Slash,
            None,
            Some(TokenKind::Comment),
        );

        // If we see a single slash after the first slash, then we're in a single line comment
        let single_line_comment_state =
            self.auto_add_transition(slash_state, Category::Slash, None, None);

        // In which we accept any character
        self.auto_add_transition(
            single_line_comment_state,
            Category::Any,
            Some(single_line_comment_state),
            None,
        );

        // Other than a newline, which ends the comment and returns a final state
        self.auto_add_transition(
            single_line_comment_state,
            Category::Newline,
            None,
            Some(TokenKind::Comment),
        );

        self
    }

    pub fn add_identifier_logic(&mut self) -> &mut Self {
        self.add_category(['_'], Category::Underscore)
            .transition()
            .to([
                Category::Letter,
                Category::HexAndLetter,
                Category::Underscore,
            ])
            .goes_to(TokenKind::Identifier)
            .to([
                Category::Letter,
                Category::HexAndLetter,
                Category::Underscore,
                Category::Digit,
            ])
            .repeated()
            .goes_to(TokenKind::Identifier)
            .done();

        self
    }

    pub fn add_number_logic(&mut self) -> &mut Self {
        self.add_category(['.'], Category::Period)
            .transition()
            .to([Category::Digit])
            .repeated()
            .goes_to(TokenKind::IntLiteral)
            .to([Category::Period])
            .to([Category::Digit])
            .repeated()
            .goes_to(TokenKind::FloatLiteral)
            .done();

        self.add_category(['#'], Category::Hashtag)
            .transition()
            .to([Category::Hashtag])
            .to([Category::Digit, Category::HexAndLetter])
            .to([Category::Digit, Category::HexAndLetter])
            .to([Category::Digit, Category::HexAndLetter])
            .to([Category::Digit, Category::HexAndLetter])
            .to([Category::Digit, Category::HexAndLetter])
            .to([Category::Digit, Category::HexAndLetter])
            .goes_to(TokenKind::ColourLiteral)
            .done();

        self
    }

    pub fn add_whitespace_logic(&mut self) -> &mut Self {
        self.add_category([' ', '\t'], Category::Whitespace) // Whitespace logic
            .transition()
            .to([Category::Whitespace])
            .repeated()
            .goes_to(TokenKind::Whitespace)
            .done();

        self
    }

    pub fn add_multi_char_rel_ops(&mut self) -> &mut Self {
        self.add_category(['<'], Category::LessThan)
            .add_category(['>'], Category::GreaterThan)
            .add_category(['='], Category::Equals)
            .add_category(['!'], Category::Exclamation)
            .add_category(['-'], Category::Minus);

        self.transition()
            .to([Category::LessThan])
            .goes_to(TokenKind::LessThan)
            .to([Category::Equals])
            .goes_to(TokenKind::LessThanEqual)
            .done();

        self.transition()
            .to([Category::Minus])
            .goes_to(TokenKind::Minus)
            .to([Category::GreaterThan])
            .goes_to(TokenKind::Arrow)
            .done();

        self.transition()
            .to([Category::GreaterThan])
            .goes_to(TokenKind::GreaterThan)
            .to([Category::Equals])
            .goes_to(TokenKind::GreaterThanEqual)
            .done();

        self.transition()
            .to([Category::Equals])
            .goes_to(TokenKind::Equals)
            .to([Category::Equals])
            .goes_to(TokenKind::EqEq)
            .done();

        self.transition()
            .to([Category::Exclamation])
            .to([Category::Equals])
            .goes_to(TokenKind::NotEqual)
            .done();

        self
    }

    pub fn build(&mut self) -> Dfsa {
        Dfsa::new(
            self.accepted_states.clone(),
            self.character_table.clone(),
            self.transition_table.clone(),
            self.state_to_token.clone(),
        )
    }
}

impl std::fmt::Display for DfsaBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Max state: {}", self.max_state)?;
        writeln!(f, "Accepted states: {:?} \n", self.accepted_states)?;

        let mut sorted_transitions: Vec<_> = self
            .transition_table
            .iter()
            .map(|((a, b), c)| (*a, *b, *c))
            .collect();

        sorted_transitions.sort_by(|a, b| a.2.cmp(&b.2));

        for (a, b, c) in sorted_transitions {
            writeln!(f, "({}, {:?}) -> {}", a, b, c)?;
        }

        writeln!(f, "Character table: {:?}", self.character_table)?;
        writeln!(f, "State to token: {:?}", self.state_to_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexing::Lexer, utils::SimpleBuffer};

    use super::*;

    use rstest::rstest;

    #[rstest]
    fn test_dfsa_builder() {
        let mut dfsa_builder = DfsaBuilder::new();

        let _dfsa = dfsa_builder.build();
    }

    #[rstest]
    fn test_dfsa_builder_to() {
        let mut dfsa_builder = DfsaBuilder::new();

        dfsa_builder
            .transition()
            .to([Category::Letter, Category::Underscore])
            .repeated()
            .to([Category::Digit])
            .done();

        println!("{}", dfsa_builder);
    }

    #[rstest]
    fn test_transition_builder() {
        let mut dfsa_builder = DfsaBuilder::new();

        dfsa_builder
            .add_category(['.'], Category::Period)
            .add_category('a'..='z', Category::Letter)
            .add_category('A'..='Z', Category::Letter)
            .add_category('0'..='9', Category::Digit)
            .add_category(['_'], Category::Underscore)
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
                ('-', Category::Minus, TokenKind::Minus),
                ('*', Category::Asterisk, TokenKind::Multiply),
                (',', Category::Comma, TokenKind::Comma),
                ('\0', Category::Eof, TokenKind::EndOfFile),
            ])
            .add_whitespace_logic()
            .add_comment_functionality()
            .add_multi_char_rel_ops()
            .add_identifier_logic()
            .add_number_logic();

        let dfsa = dfsa_builder.build();

        test_parse("bruh / /* test */ \n // test \n bruh ", Some(dfsa));
    }

    fn test_parse(string: &str, dfsa: Option<Dfsa>) {
        let fake_file = std::path::PathBuf::from("fake_file");
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(string, &fake_file, dfsa);

        match lexer.lex() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token.kind);
                }
            }
            Err(e) => {
                for error in e {
                    println!("{}", error);
                }
            }
        }
    }
}
