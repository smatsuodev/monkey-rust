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
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
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
        (TokenKind::Bang, "!"),
        (TokenKind::Minus, "-"),
        (TokenKind::Slash, "/"),
        (TokenKind::Asterisk, "*"),
        (TokenKind::Int, "5"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Int, "5"),
        (TokenKind::Lt, "<"),
        (TokenKind::Int, "10"),
        (TokenKind::Gt, ">"),
        (TokenKind::Int, "5"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::If, "if"),
        (TokenKind::LParen, "("),
        (TokenKind::Int, "5"),
        (TokenKind::Lt, "<"),
        (TokenKind::Int, "10"),
        (TokenKind::RParen, ")"),
        (TokenKind::LBrace, "{"),
        (TokenKind::Return, "return"),
        (TokenKind::True, "true"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::RBrace, "}"),
        (TokenKind::Else, "else"),
        (TokenKind::LBrace, "{"),
        (TokenKind::Return, "return"),
        (TokenKind::False, "false"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::RBrace, "}"),
        (TokenKind::Int, "10"),
        (TokenKind::Eq, "=="),
        (TokenKind::Int, "10"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Int, "10"),
        (TokenKind::NotEq, "!="),
        (TokenKind::Int, "9"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::EOF, ""),
    ];

    let mut l = Lexer::new(input);

    for (expected_kind, expected_literal) in tests.into_iter() {
        let tok = l.next_token();

        assert_eq!(tok.kind, expected_kind);
        assert_eq!(tok.literal, expected_literal);
    }
}
