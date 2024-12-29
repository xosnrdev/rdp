//! src/parser.rs

use crate::{
    ArithmeticOperator, ComparisonOperator, Expression, MatchArm, ParseError, Pattern, Program,
    Term, Token, TypeAnnotation,
};

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// The Parser struct, which processes tokens.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//-------------------------------------------------------------------------
// Implementations
//-------------------------------------------------------------------------

impl Parser {
    /// Creates a new parser instance.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parses a program.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let expression = self.parse_expression()?;
        Ok(Program { expression })
    }

    /// Parses an expression.
    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Let) => self.parse_let_expr(),
            Some(Token::If) => self.parse_if_expr(),
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Match) => self.parse_pattern_match(),
            _ => self.parse_comparison(), // Comparison includes applications
        }
    }

    /// Parses a `let` expression.
    fn parse_let_expr(&mut self) -> Result<Expression, ParseError> {
        self.consume_token(Token::Let, "Expected 'let'")?;
        let identifier = self.parse_identifier()?;
        let type_annotation = if self.match_token(Token::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };
        self.consume_token(Token::Assign, "Expected '=' in let expression")?;
        let value = self.parse_expression()?;
        self.consume_token(Token::In, "Expected 'in' in let expression")?;
        let body = self.parse_expression()?;
        Ok(Expression::LetExpr {
            identifier,
            type_annotation,
            value: Box::new(value),
            body: Box::new(body),
        })
    }

    /// Parses an `if` expression.
    fn parse_if_expr(&mut self) -> Result<Expression, ParseError> {
        self.consume_token(Token::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        self.consume_token(Token::Then, "Expected 'then' after condition")?;
        let then_branch = self.parse_expression()?;
        self.consume_token(Token::Else, "Expected 'else' after then branch")?;
        let else_branch = self.parse_expression()?;
        Ok(Expression::IfExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    /// Parses a lambda expression.
    fn parse_lambda(&mut self) -> Result<Expression, ParseError> {
        self.consume_token(Token::Lambda, "Expected '\\' for lambda")?;
        let parameter = self.parse_identifier()?;
        let type_annotation = if self.match_token(Token::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };
        self.consume_token(Token::Arrow, "Expected '->' in lambda")?;
        let body = self.parse_expression()?;
        Ok(Expression::Lambda {
            parameter,
            type_annotation,
            body: Box::new(body),
        })
    }

    /// Parses a pattern match expression.
    fn parse_pattern_match(&mut self) -> Result<Expression, ParseError> {
        self.consume_token(Token::Match, "Expected 'match'")?;
        let expression = self.parse_expression()?;
        self.consume_token(Token::With, "Expected 'with' in match")?;
        let mut arms = Vec::new();
        while self.match_token(Token::Pipe) {
            let pattern = self.parse_pattern()?;
            self.consume_token(Token::Arrow, "Expected '->' in match arm")?;
            let arm_expression = self.parse_expression()?;
            arms.push(MatchArm {
                pattern,
                expression: Box::new(arm_expression),
            });
        }
        if arms.is_empty() {
            return Err(ParseError::MissingPatternMatchArm);
        }
        Ok(Expression::PatternMatch {
            expression: Box::new(expression),
            arms,
        })
    }

    /// Parses a comparison expression.
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_logic()?;
        if let Some(operator) = match self.current_token() {
            Some(Token::Equal) => Some(ComparisonOperator::Equal),
            Some(Token::LessThan) => Some(ComparisonOperator::LessThan),
            Some(Token::GreaterThan) => Some(ComparisonOperator::GreaterThan),
            _ => None,
        } {
            self.advance();
            let right = self.parse_logic()?;
            Ok(Expression::Comparison {
                left: Box::new(left),
                operator,
                right: Some(Box::new(right)),
            })
        } else {
            Ok(left)
        }
    }

    /// Parses logical expressions (placeholder for now).
    fn parse_logic(&mut self) -> Result<Expression, ParseError> {
        self.parse_arithmetic()
    }

    /// Parses terms (identifiers, numbers, etc.).
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(Expression::Term(Term::Identifier(name))),
            Some(Token::Number(value)) => Ok(Expression::Term(Term::Number(value))),
            Some(Token::LeftParen) => {
                let expr = self.parse_expression()?;
                self.consume_token(Token::RightParen, "Expected ')' after expression")?;
                Ok(Expression::Term(Term::GroupedExpression(Box::new(expr))))
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "term".to_string(),
                found: format!("{:?}", token),
                message: "Unexpected token while parsing a term.".to_string(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(Pattern::Identifier(name)),
            Some(Token::Number(value)) => Ok(Pattern::Number(value)),
            Some(Token::LeftParen) => {
                let pattern = self.parse_pattern()?;
                self.consume_token(Token::RightParen, "Expected ')' after pattern")?;
                Ok(Pattern::Grouped(Box::new(pattern)))
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "pattern".to_string(),
                found: format!("{:?}", token),
                message: "Unexpected token while parsing a pattern.".to_string(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    /// Consumes the current token if it matches the expected token.
    fn consume_token(&mut self, expected: Token, error_message: &str) -> Result<(), ParseError> {
        if self.current_token() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", self.current_token().cloned().unwrap_or(Token::Eof)),
                message: error_message.to_string(),
            })
        }
    }

    /// Parses an identifier token.
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        if let Some(Token::Identifier(name)) = self.advance() {
            Ok(name)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self
                    .current_token()
                    .cloned()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "EOF".to_string()),
                message: "Expected an identifier".to_string(),
            })
        }
    }

    /// Parses type annotations (placeholder for now).
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        if let Some(Token::Identifier(type_name)) = self.advance() {
            match type_name.as_str() {
                "Int" => Ok(TypeAnnotation::Int),
                "Bool" => Ok(TypeAnnotation::Bool),
                "String" => Ok(TypeAnnotation::String),
                "Float" => Ok(TypeAnnotation::Float),
                _ => Err(ParseError::InvalidIdentifier(type_name)),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "type annotation".to_string(),
                found: self
                    .current_token()
                    .cloned()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "EOF".to_string()),
                message: "Expected a type annotation".to_string(),
            })
        }
    }

    /// Returns the current token without consuming it.
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Advances the parser and consumes the current token.
    fn advance(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
            self.tokens.get(self.current - 1).cloned()
        } else {
            None
        }
    }

    /// Checks if the current token matches the expected token and consumes it.
    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token() == Some(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_arithmetic(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_term()?;

        while let Some(operator) = match self.current_token() {
            Some(Token::Plus) => Some(ArithmeticOperator::Add),
            Some(Token::Minus) => Some(ArithmeticOperator::Subtract),
            Some(Token::Star) => Some(ArithmeticOperator::Multiply),
            Some(Token::Slash) => Some(ArithmeticOperator::Divide),
            _ => None,
        } {
            self.advance();
            let right = self.parse_term()?;
            left = Expression::Arithmetic {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}
