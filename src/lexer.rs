use crate::token::{Token, TokenKind};

#[cfg(test)]
mod test;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: impl ToString) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();

        l
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch {
            '=' => tok = Token::new(TokenKind::Assign, self.ch),
            ';' => tok = Token::new(TokenKind::SemiColon, self.ch),
            '(' => tok = Token::new(TokenKind::LParen, self.ch),
            ')' => tok = Token::new(TokenKind::RParen, self.ch),
            ',' => tok = Token::new(TokenKind::Comma, self.ch),
            '+' => tok = Token::new(TokenKind::Plus, self.ch),
            '{' => tok = Token::new(TokenKind::LBrace, self.ch),
            '}' => tok = Token::new(TokenKind::RBrace, self.ch),
            '\0' => tok = Token::new(TokenKind::EOF, ""),
            _ => {
                if self.is_letter() {
                    tok.literal = self.read_identifier();
                    tok.kind = TokenKind::look_up_ident(&tok.literal);
                    return tok;
                } else if self.is_digit() {
                    tok.kind = TokenKind::Int;
                    tok.literal = self.read_number();
                    return tok;
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == '_'
    }

    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}