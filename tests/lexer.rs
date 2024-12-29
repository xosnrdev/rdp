//! tests/lexer.rs

use rdp::{Lexer, ParseError, Token};

/// Tests the lexing of a simple `let` expression.
#[test]
fn test_let_expression() {
    // Arrange
    let input = "let x: Int = 42 in x + 1";
    let expected = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Identifier("Int".to_string()),
        Token::Assign,
        Token::Number(42.0),
        Token::In,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Number(1.0),
        Token::Eof,
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Assert
    assert_eq!(tokens, expected);
}

/// Tests the lexing of an `if` expression.
#[test]
fn test_if_expression() {
    // Arrange
    let input = "if x > 1 then x * 2 else x / 2";
    let expected = vec![
        Token::If,
        Token::Identifier("x".to_string()),
        Token::GreaterThan,
        Token::Number(1.0),
        Token::Then,
        Token::Identifier("x".to_string()),
        Token::Star,
        Token::Number(2.0),
        Token::Else,
        Token::Identifier("x".to_string()),
        Token::Slash,
        Token::Number(2.0),
        Token::Eof,
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Assert
    assert_eq!(tokens, expected);
}

/// Tests the lexing of a lambda expression.
#[test]
fn test_lambda_expression() {
    // Arrange
    let input = "\\x: Int -> x + 1";
    let expected = vec![
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
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Assert
    assert_eq!(tokens, expected);
}

/// Tests the lexing of a pattern match expression.
#[test]
fn test_match_expression() {
    // Arrange
    let input = "match x with | 1 -> true | _ -> false";
    let expected = vec![
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
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Assert
    assert_eq!(tokens, expected);
}

/// Tests the lexing of a complex expression with nested grouping and logical operators.
#[test]
fn test_complex_expression() {
    // Arrange
    let input = "(x + 2) * (y - 3) / (z && true)";
    let expected = vec![
        Token::LeftParen,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Number(2.0),
        Token::RightParen,
        Token::Star,
        Token::LeftParen,
        Token::Identifier("y".to_string()),
        Token::Minus,
        Token::Number(3.0),
        Token::RightParen,
        Token::Slash,
        Token::LeftParen,
        Token::Identifier("z".to_string()),
        Token::And,
        Token::Identifier("true".to_string()),
        Token::RightParen,
        Token::Eof,
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Assert
    assert_eq!(tokens, expected);
}

/// Tests error handling for an invalid token in the input.
#[test]
fn test_invalid_token() {
    // Arrange
    let input = "let x = @";

    // Act
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();

    // Assert
    assert!(result.is_err());
    match result.unwrap_err() {
        ParseError::UnexpectedToken {
            expected,
            found,
            message,
        } => {
            assert_eq!(expected, "valid token");
            assert_eq!(found, "@");
            assert_eq!(message, "Unexpected character");
        }
        _ => panic!("Unexpected error type"),
    }
}
