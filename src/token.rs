use std::fmt::Formatter;

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    #[default]
    Illegal,
    EOF,

    // 識別子 + リテラル
    Ident,
    Int,

    // 演算子
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /

    Lt,    // <
    Gt,    // >
    Eq,    // ==
    NotEq, // !=

    // デリミタ
    Comma,     // ,
    SemiColon, // ;

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // キーワード
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
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
            TokenKind::Minus => "-",
            TokenKind::Bang => "!",
            TokenKind::Asterisk => "*",
            TokenKind::Slash => "/",
            TokenKind::Eq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::Comma => ",",
            TokenKind::SemiColon => ";",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Function => "FUNCTION",
            TokenKind::Let => "LET",
            TokenKind::True => "TRUE",
            TokenKind::False => "FALSE",
            TokenKind::If => "IF",
            TokenKind::Else => "ELSE",
            TokenKind::Return => "RETURN",
        };
        write!(f, "{}", s)
    }
}

impl TokenKind {
    pub fn look_up_ident(ident: &str) -> TokenKind {
        match ident {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Ident,
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
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
