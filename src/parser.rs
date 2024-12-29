//! src/parser.rs

use crate::{
    ArithmeticOperator, ComparisonOperator, Expression, LogicOperator, MatchArm, ParseError,
    Pattern, Program, Term, Token, TypeAnnotation,
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
            _ => self.parse_comparison(),
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

    /// Parses logical expressions.
    ///
    /// Handles logical operators `&&` and `||`, respecting operator precedence.
    /// Supports multiple chained logical operations (e.g., `a && b || c`).
    fn parse_logic(&mut self) -> Result<Expression, ParseError> {
        // Parse the left-hand side arithmetic expression
        let mut left = self.parse_arithmetic()?;

        // Loop to handle multiple logical operators (left-associative)
        while let Some(token) = self.current_token() {
            // Determine if the current token is a logical operator
            let operator = match token {
                Token::And => LogicOperator::And,
                Token::Or => LogicOperator::Or,
                // Exit the loop if no logical operator is found
                _ => break,
            };

            // Consume the operator token
            self.advance();

            // Parse the right-hand side arithmetic expression
            let right = self.parse_arithmetic()?;

            // Construct the Logic expression node
            left = Expression::Logic {
                left: Box::new(left),
                operator,
                right: Some(Box::new(right)),
            };
        }

        Ok(left)
    }

    /// Parses arithmetic expressions.
    ///
    /// Handles operators like `+`, `-`, `*`, and `/`, respecting operator precedence.
    fn parse_arithmetic(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_application()?;

        while let Some(operator) = match self.current_token() {
            Some(Token::Plus) => Some(ArithmeticOperator::Add),
            Some(Token::Minus) => Some(ArithmeticOperator::Subtract),
            Some(Token::Star) => Some(ArithmeticOperator::Multiply),
            Some(Token::Slash) => Some(ArithmeticOperator::Divide),
            _ => None,
        } {
            self.advance();
            let right = self.parse_application()?;
            left = Expression::Arithmetic {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses a function application expression.
    ///
    /// An application consists of a primary term followed by one or more arguments.
    /// For example, `f x y` is parsed as Application([f, x, y]).
    fn parse_application(&mut self) -> Result<Expression, ParseError> {
        // Parse the primary term
        let mut expressions = vec![self.parse_term()?];

        // Continuously parse arguments as long as the next token starts a term
        while let Some(token) = self.current_token() {
            match token {
                // Tokens that can start a term
                Token::Identifier(_)
                | Token::Number(_)
                | Token::LeftParen
                | Token::Wildcard
                | Token::Lambda => {
                    let arg = self.parse_term()?;
                    expressions.push(arg);
                }
                // Stop if the next token cannot be part of an application
                _ => break,
            }
        }

        if expressions.len() > 1 {
            Ok(Expression::Application(expressions))
        } else {
            Ok(expressions.pop().unwrap())
        }
    }

    /// Parses terms (identifiers, numbers, etc.).
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Term(Term::Identifier(name)))
            }
            Some(Token::Number(value)) => {
                let value = *value;
                self.advance();
                Ok(Expression::Term(Term::Number(value)))
            }
            Some(Token::LeftParen) => {
                self.advance(); // Consume '('
                let expr = self.parse_expression()?;
                self.consume_token(Token::RightParen, "Expected ')' after expression")?;
                Ok(Expression::Term(Term::GroupedExpression(Box::new(expr))))
            }
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Wildcard) => {
                self.advance();
                Ok(Expression::Term(Term::Identifier("_".to_string())))
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "term".to_string(),
                found: format!("{:?}", token),
                message: "Unexpected token while parsing a term.".to_string(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    /// Parses a pattern.
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Pattern::Identifier(name))
            }
            Some(Token::Number(value)) => {
                let value = *value;
                self.advance();
                Ok(Pattern::Number(value))
            }
            Some(Token::LeftParen) => {
                self.advance(); // Consume '('
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
        if let Some(Token::Identifier(name)) = self.current_token() {
            let name = name.clone();
            self.advance();
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

    /// Parses type annotations.
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(type_name)) => {
                let type_name = type_name.clone();
                self.advance();
                match type_name.as_str() {
                    "Int" => Ok(TypeAnnotation::Int),
                    "Bool" => Ok(TypeAnnotation::Bool),
                    "String" => Ok(TypeAnnotation::String),
                    "Float" => Ok(TypeAnnotation::Float),
                    "(" => {
                        // Handle function types like (Int -> Bool)
                        self.consume_token(Token::LeftParen, "Expected '(' in function type")?;
                        let from_type = self.parse_type_annotation()?;
                        self.consume_token(Token::Arrow, "Expected '->' in function type")?;
                        let to_type = self.parse_type_annotation()?;
                        self.consume_token(Token::RightParen, "Expected ')' in function type")?;
                        Ok(TypeAnnotation::Function(
                            Box::new(from_type),
                            Box::new(to_type),
                        ))
                    }
                    _ => Err(ParseError::InvalidIdentifier(type_name)),
                }
            }
            Some(Token::LeftParen) => {
                // Handle function types like (Int -> Bool)
                self.advance(); // Consume '('
                let from_type = self.parse_type_annotation()?;
                self.consume_token(Token::Arrow, "Expected '->' in function type")?;
                let to_type = self.parse_type_annotation()?;
                self.consume_token(Token::RightParen, "Expected ')' in function type")?;
                Ok(TypeAnnotation::Function(
                    Box::new(from_type),
                    Box::new(to_type),
                ))
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "type annotation".to_string(),
                found: format!("{:?}", token),
                message: "Expected a type annotation".to_string(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    /// Returns the current token without consuming it.
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Advances the parser and consumes the current token.
    fn advance(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
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
}
