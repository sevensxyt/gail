use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Eq,
    NotEq,
    Gt,
    Lt,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "==" => Self::Eq,
            "!=" => Self::NotEq,

            "=" => Self::Assign,

            "!" => Self::Bang,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,

            "," => Self::Comma,
            ";" => Self::Semicolon,

            ">" => Self::Gt,
            "<" => Self::Lt,

            "(" => Self::Lparen,
            ")" => Self::Rparen,
            "{" => Self::Lbrace,
            "}" => Self::Rbrace,
            _ => Self::Illegal,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    pub fn from_symbol(literal: &str) -> Self {
        Self::new(TokenType::from_symbol(literal), literal.to_string())
    }

    pub fn eof() -> Self {
        Self::new(TokenType::Eof, String::new())
    }

    pub fn ident(literal: String) -> Self {
        let token_type = match literal.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,

            _ => TokenType::Ident,
        };

        Self::new(token_type, literal)
    }

    pub fn int(number: String) -> Self {
        Self::new(TokenType::Int, number)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token({:?}, {})", self.token_type, self.literal)
    }
}
