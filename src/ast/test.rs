use super::*;
use crate::token::TokenKind;

#[test]
fn test_string() {
    let program = Program {
        statements: vec![Statement::LetStatement(LetStatement::new(
            Token::new(TokenKind::Let, "let"),
            Identifier::new(Token::new(TokenKind::Ident, "myVar"), "myVar"),
            Some(Expression::Identifier(Identifier::new(
                Token::new(TokenKind::Ident, "anotherVar"),
                "anotherVar",
            ))),
        ))],
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;");
}
