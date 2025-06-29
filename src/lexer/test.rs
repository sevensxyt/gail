#[cfg(test)]
mod tests {
    use super::super::*;
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