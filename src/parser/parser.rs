use std::{error::Error, fmt::Display, str::Chars};

use strum::Display as StrumDisplay;

#[derive(Debug, StrumDisplay)]
enum TokenType {
    Number(f64),
    Arrow,
    Unit(String),
    EOF,
}

#[derive(Debug, StrumDisplay)]
enum ExpressionError {
    LexicalError(String),
    ParsingError(String)
}

impl Error for ExpressionError {}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String) -> Token {
        Token {
            token_type,
            lexeme,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({}, {})", self.token_type, self.lexeme)
    }
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: input.chars().collect::<Vec<char>>().get(0).copied(),
        }
    }
    fn advance(&mut self) {
        self.position += 1;

        if self.position > self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.get(self.position).copied();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some (' ') = self.current_char {
            self.advance();
        }
    }
}

