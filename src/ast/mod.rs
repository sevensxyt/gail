use crate::token::Token;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name: Expression,
        value: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier { token: Token, value: String },
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
