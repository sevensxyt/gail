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