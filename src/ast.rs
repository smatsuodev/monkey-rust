#[cfg(test)]
mod test;
mod util;

use crate::ast::util::define_node_enum;
use crate::token::Token;
use std::fmt::Debug;

pub trait Node: Debug + PartialEq + Eq {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

define_node_enum!(
    Statement,
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
);

define_node_enum!(
    Expression,
    Identifier,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    Boolean,
);

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.expression
            .as_ref()
            .map_or(String::new(), |e| e.to_string())
    }
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Expression>) -> ExpressionStatement {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> IntegerLiteral {
        IntegerLiteral { token, value }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "({}{})",
            self.operator,
            self.right.as_ref().map_or(String::new(), |r| r.to_string())
        )
    }
}

impl PrefixExpression {
    pub fn new(
        token: Token,
        operator: impl ToString,
        right: Option<Expression>,
    ) -> PrefixExpression {
        PrefixExpression {
            token,
            operator: operator.to_string(),
            right: right.map(Box::new),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.as_ref().map_or(String::new(), |l| l.to_string()),
            self.operator,
            self.right.as_ref().map_or(String::new(), |r| r.to_string())
        )
    }
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Option<Expression>,
        operator: impl ToString,
        right: Option<Expression>,
    ) -> InfixExpression {
        InfixExpression {
            token,
            left: left.map(Box::new),
            operator: operator.to_string(),
            right: right.map(Box::new),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Boolean {
        Boolean { token, value }
    }
}
