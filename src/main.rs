use std::io::{self, Result};

mod ast;
mod lexer;
mod parser;
mod repl;
mod test;
mod token;

fn main() -> Result<()> {
    repl::start(io::stdin().lock(), io::stdout())
}
