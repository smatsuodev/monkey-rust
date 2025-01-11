#[cfg(test)]
mod test;

use crate::ast::{
    Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
    PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::collections::HashMap;

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Option<Expression>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Option<Expression>) -> Option<Expression>;

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn<'a>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Parser<'a> {
        let mut p = Parser {
            l,
            cur_token: Token::default(),
            peek_token: Token::default(),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        p.register_prefix(TokenKind::Ident, Parser::parse_identifier);
        p.register_prefix(TokenKind::Int, Parser::parse_integer_literal);
        p.register_prefix(TokenKind::Bang, Parser::parse_prefix_expression);
        p.register_prefix(TokenKind::Minus, Parser::parse_prefix_expression);

        p.register_infix(TokenKind::Plus, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Minus, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Asterisk, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Slash, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Eq, Parser::parse_infix_expression);
        p.register_infix(TokenKind::NotEq, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Lt, Parser::parse_infix_expression);
        p.register_infix(TokenKind::Gt, Parser::parse_infix_expression);

        p.next_token();
        p.next_token();

        p
    }

    fn get_precedence(k: TokenKind) -> Precedence {
        match k {
            TokenKind::Eq | TokenKind::NotEq => Precedence::Equals,
            TokenKind::Lt | TokenKind::Gt => Precedence::LessGreater,
            TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
            TokenKind::Asterisk | TokenKind::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        Parser::get_precedence(self.peek_token.kind)
    }

    fn cur_precedence(&self) -> Precedence {
        Parser::get_precedence(self.cur_token.kind)
    }

    fn register_prefix(&mut self, k: TokenKind, f: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(k, f);
    }

    fn register_infix(&mut self, k: TokenKind, f: InfixParseFn<'a>) {
        self.infix_parse_fns.insert(k, f);
    }

    fn no_prefix_parse_fn_error(&mut self, k: TokenKind) {
        let msg = format!("no prefix parse function for {} found", k);
        self.errors.push(msg);
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, k: TokenKind) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            k, self.peek_token.kind
        );
        self.errors.push(msg)
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn cur_token_is(&self, k: TokenKind) -> bool {
        self.cur_token.kind == k
    }

    pub fn peek_token_is(&self, k: TokenKind) -> bool {
        self.peek_token.kind == k
    }

    fn expect_peek(&mut self, k: TokenKind) -> bool {
        if self.peek_token_is(k) {
            self.next_token();
            true
        } else {
            self.peek_error(k);
            false
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token.kind != TokenKind::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenKind::Ident) {
            return None;
        }

        let name = Identifier::new(self.cur_token.clone(), &self.cur_token.literal);

        if !self.expect_peek(TokenKind::Assign) {
            return None;
        }

        // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている
        while !self.cur_token_is(TokenKind::SemiColon) {
            self.next_token()
        }

        Some(Statement::LetStatement(LetStatement::new(
            token,
            name.clone(),
            None,
        )))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        self.next_token();

        // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている
        while !self.cur_token_is(TokenKind::SemiColon) {
            self.next_token()
        }

        Some(Statement::ReturnStatement(ReturnStatement::new(
            token, None,
        )))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let stmt = ExpressionStatement::new(
            self.cur_token.clone(),
            self.parse_expression(Precedence::Lowest),
        );

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Some(stmt.into())
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = match self.prefix_parse_fns.get(&self.cur_token.kind) {
            Some(f) => f,
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.kind);
                return None;
            }
        };
        let mut left_exp = prefix(self);

        while !self.peek_token_is(TokenKind::SemiColon) && precedence < self.peek_precedence() {
            let infix = match self.infix_parse_fns.get(&self.peek_token.kind) {
                Some(f) => f.clone(),
                None => return left_exp,
            };

            self.next_token();

            left_exp = infix(self, left_exp);
        }

        left_exp
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone()).into())
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let value = self.cur_token.literal.parse::<i64>().ok().or_else(|| {
            let msg = format!("could not parse {} as integer", self.cur_token.literal);
            self.errors.push(msg);
            None
        })?;

        Some(IntegerLiteral::new(token, value).into())
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix);

        Some(PrefixExpression::new(token, operator, right).into())
    }

    fn parse_infix_expression(&mut self, left: Option<Expression>) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence);

        Some(InfixExpression::new(token, left, operator, right).into())
    }
}
