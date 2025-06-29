use std::io::{self, Result};

mod lexer;
mod repl;
mod token;

fn main() -> Result<()> {
    repl::start(io::stdin().lock(), io::stdout())
}
