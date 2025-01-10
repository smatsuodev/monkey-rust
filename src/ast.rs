#[cfg(test)]
mod test;

use crate::token::Token;
use std::fmt::Debug;

pub trait Node: Debug + PartialEq + Eq {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

macro_rules! define_node_enum {
    ($enum_name:ident, $($variant:ident),*) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                $variant($variant),
            )*
        }

        impl Node for $enum_name {
            fn token_literal(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant(s) => s.token_literal(),
                    )*
                }
            }

            fn to_string(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant(s) => s.to_string(),
                    )*
                }
            }
        }
    };
}

define_node_enum!(
    Statement,
    LetStatement,
    ReturnStatement,
    ExpressionStatement
);

define_node_enum!(Expression, Identifier);

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

    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    pub value: Option<Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.value,
            self.value.as_ref().map_or(String::new(), |v| v.to_string())
        )
    }
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Option<Expression>) -> LetStatement {
        LetStatement {
            token,
            name: Box::new(name),
            value,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {};",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map_or(String::new(), |v| v.to_string())
        )
    }
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Option<Expression>) -> ReturnStatement {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Expression) -> ExpressionStatement {
        ExpressionStatement { token, expression }
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

    fn to_string(&self) -> String {
        self.value.clone()
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
