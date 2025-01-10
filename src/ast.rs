use crate::token::Token;
use std::fmt::Debug;

pub trait Node: Debug + PartialEq + Eq {
    fn token_literal(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    LetStatement(LetStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(s) => s.token_literal(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .iter()
            .nth(0)
            .map(|s| s.token_literal())
            .unwrap_or(String::new())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Expression) -> LetStatement {
        LetStatement {
            token,
            name: Box::new(name),
            value,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Identifier {
    pub fn new(token: Token, value: impl ToString) -> Identifier {
        Identifier {
            token,
            value: value.to_string(),
        }
    }
}
