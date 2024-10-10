use std::fmt::Formatter;

pub enum TokenKind {
    Illegal,
    EOF,

    // 識別子 + リテラル
    Ident,
    Int,

    // 演算子
    Assign, // =
    Plus, // +

    // デリミタ
    Comma, // ,
    SemiColon, // ;

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // キーワード
    Function,
    Let,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Illegal => "ILLEGAL",
            TokenKind::EOF => "EOF",
            TokenKind::Ident => "IDENT",
            TokenKind::Int => "INT",
            TokenKind::Assign => "=",
            TokenKind::Plus => "+",
            TokenKind::Comma => ",",
            TokenKind::SemiColon => ";",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Function => "FUNCTION",
            TokenKind::Let => "LET",
        };
        write!(f, "{}", s)
    }
}

pub struct Token {
    kind: TokenKind,
    literal: String,
}