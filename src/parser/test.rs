use crate::ast::{Node, Statement};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn test_let_statement(s: &Statement, name: &str) {
    assert_eq!(s.token_literal(), "let");

    assert!(matches!(s, Statement::LetStatement(_)));
    let let_stmt = match s {
        Statement::LetStatement(stmt) => stmt,
        _ => unreachable!(),
    };

    assert_eq!(let_stmt.name.value, name);
    assert_eq!(let_stmt.name.token_literal(), name);
}

fn check_parser_errors(p: &Parser) {
    let errors = p.errors();
    if errors.len() == 0 {
        return;
    }

    eprintln!("parser has {} errors", errors.len());
    for msg in errors {
        eprintln!("parser error: \"{}\"", msg)
    }
    panic!()
}

#[test]
fn test_let_statements() {
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 3);

    let tests = vec!["x", "y", "foobar"];
    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let_statement(stmt, tt);
    }
}
