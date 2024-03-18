use std::{cmp::max, collections::HashMap, path::Display};

use crate::core::{Token, TokenKind};

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
    DoubleQuote,
    Period,
    SingleQuote,
    Semicolon,
    Asterisk,
    Colon,
    Equals,
    EqEq,
    Slash,
    Plus,
    LessThan,
    GreaterThan,
    Minus,
    Backslash,
    Eof,
    Other,
    Comma,
    Hashtag,
    Any,
}

//
// |state, category| match (state, category) {
//     (0, Category::Whitespace) => 10,
//     (0, Category::Newline) => 20,
//     // Single character tokens
//     (0, Category::Letter | Category::Underscore) => 30,
//     (0, Category::Equals) => 31,
//     (31, Category::Equals) => 32,
//     (0, Category::LBrace) => 40,
//     (0, Category::RBrace) => 50,
//     (0, Category::LParen) => 60,
//     (0, Category::RParen) => 70,
//     (0, Category::LBracket) => 80,
//     (0, Category::RBracket) => 90,
//     (0, Category::SingleQuote) => 110,
//     (0, Category::Semicolon) => 120,
//     (0, Category::Colon) => 130,
//     (10, Category::Whitespace) => 10,
//     // Line comment
//     (0, Category::Slash) => 32,
//     (32, Category::Asterisk) => 35,
//     (35, Category::Asterisk) => 36,
//     (35, Category::Eof) => ERR_STATE,
//     (36, Category::Slash) => 37,
//     (36, Category::Eof) => ERR_STATE,
//     (35, _) => 35,
//     (32, Category::Slash) => 33,
//     (33, Category::Newline) => 34,
//     (33, Category::Eof) => ERR_STATE,
//     (33, _) => 33,
//     // Block Comment
//     // String literal logic
//     (0, Category::DoubleQuote) => 100,
//     (100, Category::DoubleQuote) => 101,
//     (100, Category::Backslash) => 102,
//     (102, Category::DoubleQuote) => 100,
//     (100, Category::Eof) => ERR_STATE,
//     (100, _) => 100,
//     // Identifier logic
//     (30, Category::Letter) => 30,
//     (30, Category::Digit) => 30,
//     (30, Category::Underscore) => 30,
//     // Integers
//     (0, Category::Digit) => 140,
//     (140, Category::Digit) => 140,
//     // Float
//     (140, Category::Period) => 150,
//     (150, Category::Digit) => 151,
//     (151, Category::Digit) => 151,
//     // Map all other characters to the error state
//     _ => ERR_STATE,
// },

// impl From<i32> for TokenKind {
//     fn from(i: i32) -> Self {
//         match i {
//             10 => TokenKind::Whitespace,
//             20 => TokenKind::Newline,
//             30 => TokenKind::Identifier(String::new()),
//             31 => TokenKind::Equals,
//             32 => TokenKind::EqEq,
//             34 => TokenKind::Comment,
//             37 => TokenKind::Comment,
//             40 => TokenKind::LBrace,
//             50 => TokenKind::RBrace,
//             60 => TokenKind::LParen,
//             70 => TokenKind::RParen,
//             80 => TokenKind::LBracket,
//             90 => TokenKind::RBracket,
//             101 => TokenKind::StringLiteral(String::new()),
//             110 => TokenKind::SingleQuote,
//             120 => TokenKind::Semicolon,
//             130 => TokenKind::Colon,
//             140 => TokenKind::IntLiteral(0),
//             151 => TokenKind::FloatLiteral(String::new()),
//             _ => TokenKind::Invalid,
//         }
//     }
// }

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

    pub fn get_token_kind(&self, state: i32) -> TokenKind {
        self.state_to_token
            .get(&state)
            .unwrap_or(&TokenKind::Invalid)
            .clone()
    }

    pub fn is_accepting(&self, state: &i32) -> bool {
        self.accepted_states.contains(state)
    }

    pub fn get_category(&self, c: char) -> Category {
        *self.character_table.get(&c).unwrap_or(&Category::Other)
    }

    pub fn delta(&self, state: i32, category: Category) -> i32 {
        // self.transition_table
        //     .get(&(state, category))
        //     .copied()
        //     .unwrap_or(-2)

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

#[derive(Debug)]
pub struct Transition<'a> {
    dfsa_builder: &'a mut DfsaBuilder,
    current: i32,
    transitions: Vec<((Category, i32), i32)>,
    final_state_token: Vec<(i32, TokenKind)>,
}

impl<'a> Transition<'a> {
    pub fn new(dfsa_builder: &'a mut DfsaBuilder) -> Self {
        Transition {
            dfsa_builder,
            current: 0,
            transitions: vec![],
            final_state_token: vec![],
        }
    }

    pub fn to<T: IntoIterator<Item = Category>>(&mut self, category: T) -> &mut Self {
        for cat in category {
            self.transitions.push((
                (
                    cat,
                    match self.current {
                        0 => 0,
                        _ => self.current + self.dfsa_builder.max_state - 1,
                    },
                ),
                self.dfsa_builder.max_state + self.current,
            ));
        }
        self.current += 1;
        self
    }

    pub fn repeated(&mut self) -> &mut Self {
        if let Some(((_, s1), s2)) = self.transitions.last().cloned() {
            let len = self.transitions.len();
            for i in (0..len).rev() {
                if self.transitions[i].0 .1 == s1 {
                    let mut new_tx = self.transitions[i];
                    new_tx.0 .1 = s2;
                    self.transitions.push(new_tx);
                } else {
                    break;
                }
            }
        }
        self
    }

    pub fn add_branch(&mut self, category: Category, next_state: i32) -> i32 {
        self.to([category]);

        self.current
    }

    pub fn goes_to(&mut self, token: TokenKind) -> &mut Self {
        self.final_state_token
            .push((self.current + self.dfsa_builder.max_state - 1, token));
        self
    }

    pub fn done(&mut self) -> &mut DfsaBuilder {
        for ((cat, state), next_state) in &self.transitions {
            self.dfsa_builder.transition_table.insert(
                (
                    match state {
                        0 => 0,
                        _ => *state,
                    },
                    *cat,
                ),
                *next_state,
            );
        }

        for (state, token) in &self.final_state_token {
            self.dfsa_builder
                .state_to_token
                .insert(*state, token.clone());
            self.dfsa_builder.accepted_states.push(*state);
        }

        self.dfsa_builder.max_state += self.current - 1;

        self.dfsa_builder
    }
}
#[derive(Debug, Clone)]
pub struct DfsaBuilder {
    pub max_state: i32,
    accepted_states: Vec<i32>,
    character_table: HashMap<char, Category>,
    transition_table: HashMap<(i32, Category), i32>,
    state_to_token: HashMap<i32, TokenKind>,
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

        // dfsa_builder.add_delimited_symbol(
        //     &[('a', Category::Letter), ('b', Category::Letter)],
        //     TokenKind::Identifier(String::new()),
        // );

        let dfsa = dfsa_builder.build();
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
            //     .add_category(['.'], Category::Period)
            //     .add_category(['_'], Category::Underscore)
            //     .add_category(['\''], Category::SingleQuote)
            //     .add_category(['"'], Category::DoubleQuote)
            //     .add_single_final_character_symbol('\n', Category::Newline, TokenKind::Newline)
            //     .add_single_final_character_symbol('{', Category::LBrace, TokenKind::LBrace)
            //     .add_single_final_character_symbol('}', Category::RBrace, TokenKind::RBrace)
            //     .add_single_final_character_symbol('(', Category::LParen, TokenKind::LParen)
            //     .add_single_final_character_symbol(')', Category::RParen, TokenKind::RParen)
            //     .add_single_final_character_symbol('[', Category::LBracket, TokenKind::LBracket)
            //     .add_single_final_character_symbol(']', Category::RBracket, TokenKind::RBracket)
            //     .add_single_final_character_symbol(';', Category::Semicolon, TokenKind::Semicolon)
            //     .add_single_final_character_symbol(':', Category::Colon, TokenKind::Colon)
            //     .add_single_final_character_symbol('=', Category::Equals, TokenKind::Equals)
            //     .add_single_final_character_symbol('<', Category::LessThan, TokenKind::LessThan)
            //     .add_single_final_character_symbol('>', Category::GreaterThan, TokenKind::GreaterThan)
            //     .add_single_final_character_symbol('+', Category::Plus, TokenKind::Plus)
            //     .add_single_final_character_symbol('-', Category::Minus, TokenKind::Minus)
            //     .add_single_final_character_symbol('*', Category::Asterisk, TokenKind::Asterisk)
            //     .add_single_final_character_symbol(',', Category::Comma, TokenKind::Comma)
            //     .add_single_final_character_symbol('#', Category::Hashtag, TokenKind::Hashtag)
            //     .add_single_final_character_symbol('\0', Category::Eof, TokenKind::EndOfFile)
            .add_category([' ', '\t'], Category::Whitespace) // Whitespace logic
            .transition()
            .to([Category::Whitespace])
            .repeated()
            .goes_to(TokenKind::Whitespace)
            .done() // Identifier logic
            .add_category('a'..='z', Category::Letter)
            .add_category('A'..='Z', Category::Letter)
            .add_category('0'..='9', Category::Digit)
            .transition()
            .to([Category::Letter, Category::Underscore])
            .goes_to(TokenKind::Identifier(String::new()))
            .to([Category::Letter, Category::Underscore, Category::Digit])
            .repeated()
            .goes_to(TokenKind::Identifier(String::new()))
            .done()
            .transition()
            .to([Category::Digit])
            .repeated()
            .goes_to(TokenKind::IntLiteral(0))
            .to([Category::Period])
            .to([Category::Digit])
            .repeated()
            .goes_to(TokenKind::FloatLiteral(String::new()))
            .done();

        dfsa_builder
            .add_single_final_character_symbol('\n', Category::Newline, TokenKind::Newline)
            .add_category(['\0'], Category::Eof)
            .add_category(['/'], Category::Slash)
            .add_category(['*'], Category::Asterisk)
            .transition()
            .to([Category::Slash])
            // .goes_to(TokenKind::Divide)
            .done();

        let slash_state = dfsa_builder.max_state;

        let in_multiline_comment_state =
            dfsa_builder.auto_add_transition(slash_state, Category::Asterisk, None, None);

        //TODO: ADD THIS TO MAIN LExer
        dfsa_builder.auto_add_transition(in_multiline_comment_state, Category::Eof, Some(-2), None);
        dfsa_builder.auto_add_transition(
            in_multiline_comment_state,
            Category::Any,
            Some(in_multiline_comment_state),
            None,
        );

        let single_line_comment_state =
            dfsa_builder.auto_add_transition(slash_state, Category::Slash, None, None);

        dfsa_builder.auto_add_transition(
            single_line_comment_state,
            Category::Newline,
            None,
            Some(TokenKind::Comment),
        );

        dfsa_builder.auto_add_transition(
            single_line_comment_state,
            Category::Any,
            Some(single_line_comment_state),
            None,
        );

        let multiline_comment_end_asterisk_state = dfsa_builder.auto_add_transition(
            in_multiline_comment_state,
            Category::Asterisk,
            None,
            None,
        );

        dfsa_builder.auto_add_transition(
            multiline_comment_end_asterisk_state,
            Category::Eof,
            Some(-2),
            None,
        );

        dfsa_builder.auto_add_transition(
            multiline_comment_end_asterisk_state,
            Category::Slash,
            None,
            Some(TokenKind::Comment),
        );

        println!("{}", dfsa_builder);

        let dfsa = dfsa_builder.build();

        test_parse("bruh /* test */ \n // test \n bruh ", Some(dfsa));
    }

    fn test_parse(string: &str, dfsa: Option<Dfsa>) {
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(string, dfsa);

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
