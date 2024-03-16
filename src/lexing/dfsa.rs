use std::collections::HashMap;

use crate::core::TokenKind;

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
    CrocLeft,
    CrocRight,
    Minus,
    Backslash,
    Eof,
    Other,
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

pub struct Dfsa {
    accepted_states: Vec<i32>,
    character_table: HashMap<char, Category>,
    transition_table: HashMap<(i32, Category), i32>,
    state_to_token: HashMap<i32, TokenKind>,
}

impl Dfsa {
    pub fn new(accepted_states: Vec<i32>) -> Self {
        let character_table: HashMap<char, Category> = ('a'..='z')
            .map(|c| (c, Category::Whitespace))
            .chain(('A'..='Z').map(|c| (c, Category::Letter)))
            .chain(('0'..='9').map(|c| (c, Category::Digit)))
            .chain(vec![
                ('_', Category::Underscore),
                (' ', Category::Whitespace),
                ('\t', Category::Whitespace),
                ('\n', Category::Newline),
                ('{', Category::LBrace),
                ('/', Category::Slash),
                ('}', Category::RBrace),
                ('.', Category::Period),
                ('=', Category::Equals),
                ('+', Category::Plus),
                ('-', Category::Minus),
                ('<', Category::CrocLeft),
                ('>', Category::CrocRight),
                ('*', Category::Asterisk),
                ('(', Category::LParen),
                (')', Category::RParen),
                (':', Category::Colon),
                ('\\', Category::Backslash),
                ('[', Category::LBracket),
                (']', Category::RBracket),
                ('"', Category::DoubleQuote),
                ('\'', Category::SingleQuote),
                (';', Category::Semicolon),
                ('\0', Category::Eof),
            ])
            .collect();

        // let transition_table: HashMap<(i32, Category), i32> = vec![
        //     (0, Category::Whitespace) => 10,
        //     (0, Category::Newline) => 20,
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
        //     (0, Category::Slash) => 32,
        //     (32, Category::Asterisk) => 35,
        //     (35, Category::Asterisk) => 36,
        //     (35, Category::Eof) => -2,
        //     (36, Category::Slash) => 37,
        //     (36, Category::Eof) => -2,
        //     (35, _) => 35,
        //     (32, Category::Slash) => 33,
        //     (33, Category::Newline) => 34,
        //     (33, Category::Eof) => -2,
        //     (33, _) => 33,
        //     (0, Category::DoubleQuote) => 100,
        //     (100, Category::DoubleQuote) => 101,
        //     (100, Category::Backslash) => 102,
        //     (102, Category::DoubleQuote) => 100,
        //     (100, Category::Eof) => -2,
        //     (100, _) => 100,
        //     (30, Category::Letter) => 30,
        //     (30, Category::Digit) => 30,
        //     (30, Category::Underscore) => 30,
        //     (0, Category::Digit) => 140,
        //     (140, Category::Digit) => 140,
        //     (140, Category::Period) => 150,
        //     (150, Category::Digit)
        // ];

        Dfsa {
            accepted_states,
            character_table,
            transition_table: HashMap::new(),
            state_to_token: HashMap::new(),
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
        self.transition_table
            .get(&(state, category))
            .copied()
            .unwrap_or(-2)
    }
}

#[derive(Debug, Clone)]
pub struct Transition {
    start: i32,
    current: i32,
    transitions: Vec<((Category, i32), i32)>,
}

impl Transition {
    pub fn new(start: i32) -> Self {
        Transition {
            start: start + 1,
            current: 0,
            transitions: vec![],
        }
    }

    pub fn to<T: IntoIterator<Item = Category>>(&mut self, category: T) -> &mut Self {
        for cat in category {
            self.transitions
                .push(((cat, self.current), self.current + self.start));
        }
        self.current += 1;
        self
    }

    pub fn done(&self) -> Self {
        self.clone()
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
    /// Creates a new DfsaBuilder. By default, the builder will have the following categories:
    /// - Letters: a-z, A-Z
    /// - Digits: 0-9
    /// - Whitespace:  `space`` and `tab`
    pub fn new() -> Self {
        let dfsa_builder = DfsaBuilder {
            max_state: 0,
            accepted_states: vec![],
            character_table: HashMap::new(),
            transition_table: HashMap::new(),
            state_to_token: HashMap::new(),
        };

        // dfsa_builder
        //     .add_category('a'..='z', Category::Letter)
        //     .add_category('A'..='Z', Category::Letter)
        //     .add_category('0'..='9', Category::Digit)
        //     .add_category(['\0'], Category::Eof)
        //     .add_category([' ', '\t'], Category::Whitespace);

        dfsa_builder
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

    pub fn transition(&self) -> Transition {
        Transition::new(self.max_state)
    }

    pub fn add_built_transition(&mut self, transition: Transition) -> &mut Self {
        for ((cat, state), next_state) in transition.transitions {
            self.transition_table.insert(
                (
                    match state {
                        0 => 0,
                        _ => state + transition.start - 1,
                    },
                    cat,
                ),
                next_state,
            );
        }
        self.max_state += transition.current;
        self
    }

    pub fn auto_add_transition<T: IntoIterator<Item = Category>>(
        mut self,
        state: i32,
        category: T,
    ) -> i32 {
        self.max_state += 1;
        let new_state = self.max_state;

        self.add_transition(state, category, new_state);

        new_state
    }

    pub fn add_single_character_symbol(&mut self, character: char, category: Category) -> i32 {
        self.max_state += 1;
        let new_state = self.max_state;

        self.character_table.insert(character, category);
        self.add_transition(0, [category], new_state);

        new_state
    }

    pub fn add_single_final_character_symbol(
        &mut self,
        character: char,
        category: Category,
        token_kind: TokenKind,
    ) -> &mut Self {
        self.max_state += 1;
        let new_state = self.max_state;

        self.character_table.insert(character, category);
        self.state_to_token.insert(new_state, token_kind);
        self.accepted_states.push(new_state);
        self.add_transition(0, [category], new_state);

        self
    }

    /// Adds a repeatable final character symbol to the dfsa
    /// This means that the character can be repeated any number of times
    /// and the token will be accepted
    /// Example: Whitespace
    pub fn add_repeatable_final_character_symbol(
        mut self,
        character: char,
        category: Category,
        token_kind: TokenKind,
    ) -> Self {
        self.max_state += 1;
        let new_state = self.max_state;

        self.character_table.insert(character, category);
        self.accepted_states.push(new_state);
        self.clone().add_transition(0, [category], new_state);
        self.clone()
            .add_transition(new_state, [category], new_state);
        self.state_to_token.insert(new_state, token_kind);

        self
    }

    // take a character range as a parameter
    pub fn add_category<T: IntoIterator<Item = char>>(
        mut self,
        range: T,
        category: Category,
    ) -> Self {
        range.into_iter().for_each(|c| {
            self.character_table.insert(c, category);
        });

        self
    }

    pub fn add_multi_char_final_symbol(
        mut self,
        symbol: &[(char, Category)],
        token_kind: TokenKind,
    ) -> Self {
        let mut current_state = 0;
        for (c, cat) in symbol {
            self.max_state += 1;
            let new_state = self.max_state;
            self.character_table.insert(*c, *cat);
            self.clone()
                .add_transition(current_state, [*cat], new_state);
            current_state = new_state;
        }

        self.accepted_states.push(current_state);
        self.state_to_token.insert(current_state, token_kind);

        self
    }

    pub fn build(&mut self) -> Dfsa {
        Dfsa {
            accepted_states: self.accepted_states.clone(),
            transition_table: self.transition_table.clone(),
            character_table: self.character_table.clone(),
            state_to_token: self.state_to_token.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn test_transition_builder() {
        let mut dfsa_builder = DfsaBuilder::new();

        dfsa_builder.add_single_final_character_symbol(
            '{',
            Category::LBrace,
            TokenKind::Identifier(String::new()),
        );

        dfsa_builder.add_single_final_character_symbol(
            '}',
            Category::RBrace,
            TokenKind::Identifier(String::new()),
        );

        let t = dfsa_builder
            .transition()
            .to([Category::Letter, Category::Underscore])
            .to([Category::Digit])
            .done();

        dfsa_builder.add_built_transition(t);

        // println!("{:?}", a);
        println!("{:?}", dfsa_builder);
    }
}
