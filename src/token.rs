use std::fmt::Formatter;

#[derive(Default, Debug, Eq, PartialEq)]
pub enum TokenKind {
    #[default]
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

impl TokenKind {
    pub fn look_up_ident(ident: &str) -> TokenKind {
        match ident {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            _ => TokenKind::Ident
        }
    }
}

#[derive(Default)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: impl ToString) -> Token {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }
}