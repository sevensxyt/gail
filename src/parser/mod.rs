use crate::{
    ast::{Expression, Program, Statement},
    lexer::{Lexer, LexerError},
    token::{Token, TokenType},
};

#[derive(Debug)]
enum ParserError {
    UnexpectedToken(Token),
    LexerError(LexerError),
    UnexpectedTokenType(TokenType, TokenType),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            Self::LexerError(error) => write!(f, "Error lexing program: {:?}", error),
            Self::UnexpectedTokenType(expected, received) => write!(
                f,
                "Expected token type of {:?}, received {:?}",
                expected, received
            ),
        }
    }
}

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    errors: Vec<String>,

    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Parser<'a>, LexerError> {
        let first = lexer.next_token()?;
        let second = lexer.next_token()?;

        Ok(Self {
            lexer,
            errors: vec![],
            curr_token: first,
            peek_token: second,
        })
    }

    fn next_token(&mut self) -> Result<(), ParserError> {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().map_err(ParserError::LexerError)?;
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements: Vec<Statement> = vec![];

        while self.curr_token.token_type != TokenType::Eof {
            statements.push(self.parse_statement()?);
            self.next_token()?;
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        Ok(match self.curr_token.token_type {
            TokenType::Let => self.parse_let_statement()?,
            _ => {
                return Err(ParserError::UnexpectedToken(self.curr_token.clone()));
            }
        })
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let let_token = self.curr_token.clone(); 
        
        self.expect_peek(TokenType::Ident)?;
        let name = Expression::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        self.expect_peek(TokenType::Assign)?;

        while !self.curr_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }

        let dummy_value = Expression::Identifier {
            token: self.curr_token.clone(),
            value: String::new(),
        };

        Ok(Statement::Let {
            token: let_token,
            name,
            value: dummy_value,
        })
    }

    fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if self.peek_token_is(token_type) {
            self.next_token()?;
            Ok(())
        } else {
            Err(ParserError::UnexpectedTokenType(
                token_type,
                self.peek_token.token_type,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Statement},
        fail,
        lexer::Lexer,
        parser::Parser,
        token::TokenType,
    };

    #[test]
    fn test_let_statements() {
        let input = r#"
                        let x = 5;
                        let y = 10;
                        let foobar = 838383;
                    "#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer).expect("failed to create parser");

        let program = parser.parse_program().expect("failed to parse program");

        assert_eq!(
            program.statements.len(),
            3,
            "expected 3 statements, got {}",
            program.statements.len()
        );

        let tests = ["x", "y", "foobar"];

        for (i, &expected_ident) in tests.iter().enumerate() {
            assert!(
                test_let_statement(&program.statements[i], expected_ident),
                "test_let_statement failed for statement {}",
                i
            );
        }
    }

    fn test_let_statement(stmt: &Statement, expected_name: &str) -> bool {
        match stmt {
            Statement::Let { token, name, .. } => {
                if token.token_type != TokenType::Let {
                    fail!("Token type not 'Let'. got={:?}", token);
                }

                match name {
                    Expression::Identifier { token, value } => {
                        if token.token_type != TokenType::Ident {
                            fail!("Token type not 'Ident'. got={:?}", token);
                        }

                        if value != expected_name {
                            fail!(
                                "Identifier value wrong. expected={}, got={}",
                                expected_name,
                                value
                            );
                        }
                    }
                    _ => {
                        fail!("Expected identifier expression, got {:?}", name);
                    }
                }

                true
            }
            _ => {
                fail!("Not Let statement. Got {:?}", stmt)
            }
        }
    }
}
