use super::*;
use crate::ast::*;
use crate::lexer::Lexer;
use std::any::Any;

fn test_let_statement(s: &Statement, name: &str) {
    assert_eq!(s.token_literal(), "let");

    assert!(matches!(s, Statement::LetStatement(_)));
    let let_stmt: LetStatement = s.try_into().unwrap();

    assert_eq!(let_stmt.name.value, name);
    assert_eq!(let_stmt.name.token_literal(), name);
}

fn test_integer_literal(il: Expression, value: i64) {
    assert!(matches!(il, Expression::IntegerLiteral(_)));
    let int: IntegerLiteral = il.try_into().unwrap();
    assert_eq!(int.value, value);
    assert_eq!(int.token_literal(), value.to_string());
}

fn test_identifier(exp: Expression, value: String) {
    assert!(matches!(exp, Expression::Identifier(_)));
    let ident: Identifier = exp.try_into().unwrap();
    assert_eq!(ident.value, value);
    assert_eq!(ident.token_literal(), value);
}

fn test_boolean_literal(exp: Expression, value: bool) {
    assert!(matches!(exp, Expression::Boolean(_)));
    let boolean: Boolean = exp.try_into().unwrap();
    assert_eq!(boolean.value, value);
    assert_eq!(boolean.token_literal(), value.to_string());
}

macro_rules! test_literal_expression {
    ($exp:expr, $value:expr) => {
        if let Some(value) = ($value as &dyn Any).downcast_ref::<i32>() {
            test_integer_literal($exp, *value as i64);
        } else if let Some(value) = ($value as &dyn Any).downcast_ref::<i64>() {
            test_integer_literal($exp, *value);
        } else if let Some(value) = ($value as &dyn Any).downcast_ref::<&str>() {
            test_identifier($exp, value.to_string());
        } else if let Some(value) = ($value as &dyn Any).downcast_ref::<bool>() {
            test_boolean_literal($exp, *value);
        } else {
            panic!(
                "type of value not handled. got={:?}, type={:?}",
                $value,
                $value.type_id()
            );
        }
    };
}

macro_rules! test_infix_expression {
    ($exp:expr, $left:expr, $op:expr, $right:expr) => {
        assert!(matches!($exp, Expression::InfixExpression(_)));
        let infix: InfixExpression = $exp.try_into().unwrap();
        test_literal_expression!(*infix.left.unwrap(), $left);
        assert_eq!(infix.operator, $op);
        test_literal_expression!(*infix.right.unwrap(), $right);
    };
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

#[test]
fn test_return_statements() {
    let input = "
return 5;
return 10;
return 993322;
";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 3);

    for stmt in program.statements {
        assert!(matches!(stmt, Statement::ReturnStatement(_)));
        assert_eq!(stmt.token_literal(), "return");
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
    test_literal_expression!(stmt.expression.unwrap(), &"foobar");
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
    test_literal_expression!(stmt.expression.unwrap(), &5);
}

#[test]
fn test_parsing_prefix_expressions() {
    let prefix_tests: Vec<(&str, &str, &dyn Any)> = vec![
        ("!5;", "!", &5),
        ("-15;", "-", &15),
        ("!true;", "!", &true),
        ("!false;", "!", &false),
    ];

    for (input, op, value) in prefix_tests {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);

        let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
        let exp: PrefixExpression = stmt.expression.unwrap().try_into().unwrap();
        assert_eq!(exp.operator, op);
        test_literal_expression!(*exp.right.unwrap(), value);
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let infix_tests: Vec<(&str, &dyn Any, &str, &dyn Any)> = vec![
        ("5 + 5;", &5, "+", &5),
        ("5 - 5;", &5, "-", &5),
        ("5 * 5;", &5, "*", &5),
        ("5 / 5;", &5, "/", &5),
        ("5 > 5;", &5, ">", &5),
        ("5 < 5;", &5, "<", &5),
        ("5 == 5;", &5, "==", &5),
        ("5 != 5;", &5, "!=", &5),
        ("true == true", &true, "==", &true),
        ("true != false", &true, "!=", &false),
        ("false == false", &false, "==", &false),
    ];

    for (input, left, op, right) in infix_tests {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);

        let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
        test_infix_expression!(stmt.expression.clone().unwrap(), left, op, right);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];

    for (input, expected) in tests {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.to_string(), expected);
    }
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
    let exp: IfExpression = stmt.expression.unwrap().try_into().unwrap();
    test_infix_expression!(*exp.condition.as_ref().unwrap().clone(), &"x", "<", &"y");
    assert_eq!(exp.consequence.statements.len(), 1);
    let consequence: ExpressionStatement = (&exp.consequence.statements[0]).try_into().unwrap();
    test_identifier(consequence.expression.unwrap(), "x".to_string());
    assert_eq!(exp.alternative, None);
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt: ExpressionStatement = (&program.statements[0]).try_into().unwrap();
    let exp: IfExpression = stmt.expression.unwrap().try_into().unwrap();
    test_infix_expression!(*exp.condition.as_ref().unwrap().clone(), &"x", "<", &"y");
    assert_eq!(exp.consequence.statements.len(), 1);
    let consequence: ExpressionStatement = (&exp.consequence.statements[0]).try_into().unwrap();
    test_identifier(consequence.expression.unwrap(), "x".to_string());
    assert_eq!(exp.alternative.as_ref().unwrap().statements.len(), 1);
    let alternative: ExpressionStatement = (&exp.alternative.as_ref().unwrap().statements[0])
        .try_into()
        .unwrap();
    test_identifier(alternative.expression.unwrap(), "y".to_string());
}
