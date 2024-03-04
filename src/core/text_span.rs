use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub from_line: usize,
    pub to_line: usize,
    pub from_col: usize,
    pub to_col: usize,
    pub lexeme: String,
}

impl TextSpan {
    pub fn new(
        from_line: usize,
        to_line: usize,
        from_col: usize,
        to_col: usize,
        lexeme: &str,
    ) -> TextSpan {
        TextSpan {
            from_line,
            to_line,
            from_col,
            to_col,
            lexeme: lexeme.to_string(),
        }
    }
}

impl Display for TextSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({},{}):({},{}):{}",
            self.from_line, self.from_col, self.to_line, self.to_col, self.lexeme
        )
    }
}
