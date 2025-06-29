use crate::{lexer::Lexer, token::TokenType};
use std::io::Result;

const PROMPT: &'static str = ">> ";

pub fn start<R: std::io::BufRead, W: std::io::Write>(mut input: R, mut output: W) -> Result<()> {
    loop {
        write!(output, "{}", PROMPT)?;
        output.flush()?;

        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Ok(());
        }

        let mut lexer = Lexer::new(&line.trim_end());

        loop {
            match lexer.next_token() {
                Ok(t) if t.token_type == TokenType::Eof => {
                    break;
                }
                Ok(t) => {
                    writeln!(output, "{}", t)?;
                }
                Err(e) => {
                    writeln!(output, "Error: {:?}", e)?;
                    break;
                }
            }
        }
    }
}
