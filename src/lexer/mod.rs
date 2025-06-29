use std::num::ParseIntError;

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum LexerError {
    InvalidNumber(String, ParseIntError),
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

static SYMBOL_TOKENS: &[(&str, TokenType)] = &[
    ("==", TokenType::Eq),
    ("!=", TokenType::NotEq),
    ("=", TokenType::Assign),
    (";", TokenType::Semicolon),
    (",", TokenType::Comma),
    ("!", TokenType::Bang),
    ("+", TokenType::Plus),
    ("-", TokenType::Minus),
    ("*", TokenType::Asterisk),
    ("/", TokenType::Slash),
    (">", TokenType::Gt),
    ("<", TokenType::Lt),
    ("{", TokenType::Lbrace),
    ("}", TokenType::Rbrace),
    ("(", TokenType::Lparen),
    (")", TokenType::Rparen),
];

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let token = if let Some(c) = self.ch {
            if let Some(symbol) = self.read_symbol() {
                Token::from_symbol(&symbol)
            } else if c.is_ascii_alphabetic() {
                Token::ident(self.read_identifier())
            } else if c.is_ascii_digit() {
                Token::int(self.read_number()?)
            } else {
                self.read_char();
                Token::new(TokenType::Illegal, c.to_string())
            }
        } else {
            Token::eof()
        };

        Ok(token)
    }

    fn read_char(&mut self) {
        self.ch = self.input[self.read_position..].chars().next();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_n(&mut self, n: usize) {
        for _ in 0..n {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while matches!(self.ch, Some(c) if c.is_ascii_alphabetic()) {
            self.read_char();
        }

        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> Result<String, LexerError> {
        let start = self.position;

        while matches!(self.ch, Some(c) if c.is_ascii_digit()) {
            self.read_char();
        }

        let number = &self.input[start..self.position];
        let _ = number
            .parse::<i32>()
            .map_err(|e| LexerError::InvalidNumber(number.to_string(), e));
        Ok(number.to_string())
    }

    fn read_symbol(&mut self) -> Option<String> {
        let curr = &self.input[self.position..];
        SYMBOL_TOKENS.iter().find_map(|(symbol, _)| {
            curr.starts_with(symbol).then(|| {
                self.read_n(symbol.len());
                symbol.to_string()
            })
        })
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, Some(c) if matches!(c, ' ' | '\t' | '\n' | '\r')) {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test;
