use std::num::ParseIntError;

use crate::token::{Token, TokenType};

#[derive(Debug)]
#[allow(dead_code)]
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
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let expected = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::If, "if"),
            (TokenType::Lparen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::Ident, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::Ident, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            //
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            //
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_token_type, expected_literal)) in expected.iter().enumerate() {
            let received = lexer.next_token().unwrap();

            assert_eq!(
                expected_token_type, &received.token_type,
                "test {}: token_type mismatch, expected {:?}, got {:?}",
                i, expected_token_type, received.token_type
            );

            assert_eq!(
                expected_literal, &received.literal,
                "test {}: literal mismatch, expected {:?}, got {:?}",
                i, expected_literal, received.literal
            );
        }
    }
}
