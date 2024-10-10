use crate::lexer::Lexer;
use crate::token::TokenKind;

#[test]
fn test_next_token() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
    ";

    let tests = vec![
        (TokenKind::Let, "let"),
        (TokenKind::Ident, "five"),
        (TokenKind::Assign, "="),
        (TokenKind::Int, "5"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Let, "let"),
        (TokenKind::Ident, "ten"),
        (TokenKind::Assign, "="),
        (TokenKind::Int, "10"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Let, "let"),
        (TokenKind::Ident, "add"),
        (TokenKind::Assign, "="),
        (TokenKind::Function, "fn"),
        (TokenKind::LParen, "("),
        (TokenKind::Ident, "x"),
        (TokenKind::Comma, ","),
        (TokenKind::Ident, "y"),
        (TokenKind::RParen, ")"),
        (TokenKind::LBrace, "{"),
        (TokenKind::Ident, "x"),
        (TokenKind::Plus, "+"),
        (TokenKind::Ident, "y"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::RBrace, "}"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Let, "let"),
        (TokenKind::Ident, "result"),
        (TokenKind::Assign, "="),
        (TokenKind::Ident, "add"),
        (TokenKind::LParen, "("),
        (TokenKind::Ident, "five"),
        (TokenKind::Comma, ","),
        (TokenKind::Ident, "ten"),
        (TokenKind::RParen, ")"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::EOF, ""),
    ];

    let mut l = Lexer::new(input);

    for (expectedKind, expectedLiteral) in tests.into_iter() {
        let tok = l.next_token();

        assert_eq!(tok.kind, expectedKind);
        assert_eq!(tok.literal, expectedLiteral);
    }
}