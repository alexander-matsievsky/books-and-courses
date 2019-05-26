use crate::token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(token::Token),
    Int(token::Token),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(token::Token, Expression),
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
