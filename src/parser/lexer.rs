use std::{error::Error, fmt::Display, num::ParseFloatError};

use strum::Display as StrumDisplay;

use crate::units::main::Unit;

#[derive(Debug, StrumDisplay)]
enum TokenType {
    Number(f64),
    Arrow,
    Unit(Unit),
    EOF,
}

#[derive(Debug, StrumDisplay)]
enum ExpressionError {
    LexicalError(String),
    ParsingError(ParseFloatError)
}

impl Error for ExpressionError {}

#[derive(Debug)]
pub struct Token {
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

    fn peek(&self) -> &char {
        self.input.get(self.position + 1).unwrap()
    }

    fn is_not_end(&self) -> bool {
        self.input.get(self.position + 1).is_some()
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

    fn get_number(&mut self) -> Result<f64, ExpressionError> {
        let mut result = String::new();

        while self.current_char.is_some() && self.current_char.unwrap().is_numeric() {
            result += self.current_char.unwrap().to_string().as_str();
            self.advance();
        }

        result.parse().map_err(ExpressionError::ParsingError)
    }

    fn get_unit(&mut self) -> Unit {
        let mut result = String::new();

        while self.current_char.is_some() && self.current_char.unwrap().is_alphabetic() {
            result += self.current_char.unwrap().to_string().as_str();
            self.advance();
        }

        Unit::from(result)
    }

    fn get_next_token(&mut self) -> Result<Token, ExpressionError> {
        while let Some(c) = self.current_char {
            match c {
                ' ' => self.skip_whitespace(),
                d if d.is_numeric() => {
                    return Ok(Token::new(TokenType::Number(self.get_number()?), c.to_string()));
                },
                u if u.is_alphabetic() => {
                    let unit = self.get_unit();
                    let lexeme = unit.to_string();
                    return  Ok(Token::new( TokenType::Unit(unit), lexeme));
                },
                '-' if self.is_not_end() && self.peek() == &'>' => {
                    self.advance();
                    self.advance();
                    return Ok(Token::new(TokenType::Arrow, "->".to_string()));
                },
                c => {
                    return Err(ExpressionError::LexicalError(c.to_string()));
                }
            }
        }
        
        Ok(Token::new(TokenType::EOF, "EOF".to_string()))
    }
}

