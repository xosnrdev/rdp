//! tests/parser.rs

use rdp::{
    ArithmeticOperator, ComparisonOperator, Expression, MatchArm, ParseError, Parser, Pattern,
    Program, Term, Token, TypeAnnotation,
};

/// Tests parsing of a `let` expression.
#[test]
fn test_program_parsing_with_let() {
    // Arrange
    let tokens = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Identifier("Int".to_string()),
        Token::Assign,
        Token::Number(42.0),
        Token::In,
        Token::Identifier("x".to_string()),
        Token::Eof,
    ];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::LetExpr {
                identifier: "x".to_string(),
                type_annotation: Some(TypeAnnotation::Int),
                value: Box::new(Expression::Term(Term::Number(42.0))),
                body: Box::new(Expression::Term(Term::Identifier("x".to_string()))),
            }
        }
    );
}

/// Tests parsing of an `if` expression.
#[test]
fn test_program_parsing_with_if() {
    // Arrange
    let tokens = vec![
        Token::If,
        Token::Identifier("x".to_string()),
        Token::GreaterThan,
        Token::Number(0.0),
        Token::Then,
        Token::Number(1.0),
        Token::Else,
        Token::Number(2.0),
        Token::Eof,
    ];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::IfExpr {
                condition: Box::new(Expression::Comparison {
                    left: Box::new(Expression::Term(Term::Identifier("x".to_string()))),
                    operator: ComparisonOperator::GreaterThan,
                    right: Some(Box::new(Expression::Term(Term::Number(0.0)))),
                }),
                then_branch: Box::new(Expression::Term(Term::Number(1.0))),
                else_branch: Box::new(Expression::Term(Term::Number(2.0))),
            }
        }
    );
}

/// Tests parsing of a lambda expression.
#[test]
fn test_program_parsing_with_lambda() {
    // Arrange
    let tokens = vec![
        Token::Lambda,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Identifier("Int".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Number(1.0),
        Token::Eof,
    ];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::Lambda {
                parameter: "x".to_string(),
                type_annotation: Some(TypeAnnotation::Int),
                body: Box::new(Expression::Arithmetic {
                    left: Box::new(Expression::Term(Term::Identifier("x".to_string()))),
                    operator: ArithmeticOperator::Add,
                    right: Box::new(Expression::Term(Term::Number(1.0))),
                }),
            }
        }
    );
}

/// Tests parsing of a pattern match expression.
#[test]
fn test_program_parsing_with_pattern_match() {
    // Arrange
    let tokens = vec![
        Token::Match,
        Token::Identifier("x".to_string()),
        Token::With,
        Token::Pipe,
        Token::Number(1.0),
        Token::Arrow,
        Token::Identifier("true".to_string()),
        Token::Pipe,
        Token::Identifier("_".to_string()),
        Token::Arrow,
        Token::Identifier("false".to_string()),
        Token::Eof,
    ];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::PatternMatch {
                expression: Box::new(Expression::Term(Term::Identifier("x".to_string()))),
                arms: vec![
                    MatchArm {
                        pattern: Pattern::Number(1.0),
                        expression: Box::new(Expression::Term(Term::Identifier(
                            "true".to_string()
                        ))),
                    },
                    MatchArm {
                        pattern: Pattern::Identifier("_".to_string()),
                        expression: Box::new(Expression::Term(Term::Identifier(
                            "false".to_string()
                        ))),
                    },
                ],
            }
        }
    );
}

/// Tests parsing of a comparison expression.
#[test]
fn test_program_parsing_with_comparison() {
    // Arrange
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Equal,
        Token::Number(42.0),
        Token::Eof,
    ];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::Comparison {
                left: Box::new(Expression::Term(Term::Identifier("x".to_string()))),
                operator: ComparisonOperator::Equal,
                right: Some(Box::new(Expression::Term(Term::Number(42.0)))),
            }
        }
    );
}

/// Tests handling of an empty program.
#[test]
fn test_empty_program() {
    // Arrange
    let tokens = vec![Token::Eof];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ParseError::UnexpectedToken {
            expected: "term".to_string(),
            found: "Eof".to_string(),
            message: "Unexpected token while parsing a term.".to_string(),
        }
    );
}

/// Tests parsing of a single term application.
#[test]
fn test_single_term_application() {
    // Arrange
    let tokens = vec![Token::Identifier("x".to_string()), Token::Eof];

    // Act
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    // Assert
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(
        program,
        Program {
            expression: Expression::Term(Term::Identifier("x".to_string())),
        }
    );
}