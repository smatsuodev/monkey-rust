#[cfg(test)]
mod test;

use crate::ast::{Expression, Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Parser<'a> {
        let mut p = Parser {
            l,
            cur_token: Token::default(),
            peek_token: Token::default(),
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        p
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

    fn peek_token_is(&self, k: TokenKind) -> bool {
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
            _ => None,
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

        while !self.cur_token_is(TokenKind::SemiColon) {
            self.next_token()
        }

        Some(Statement::LetStatement(LetStatement::new(
            token,
            name.clone(),
            Expression::Identifier(name),
        )))
    }
}
