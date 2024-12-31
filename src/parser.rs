//! src/parser.rs

/*******************************************************************************
 *                          RECURSIVE DESCENT PARSER
 *-------------------------------------------------------------------------------
 * This parser handles a functional language grammar, producing an AST for use
 * in interpretation or code generation. It follows a top-down approach,
 * mapping each EBNF rule to a dedicated function, and respects operator
 * precedence through the chaining of parse_* methods.
 *
 * Key grammar constructs:
 *   - Let, If, Lambda, and Match expressions
 *   - Comparisons, logic, arithmetic, and application expressions
 *   - Function composition with the dot operator (.)
 *   - Optional type annotations (e.g. `x: Int`)
 *
 * This version also includes a `parse_expression_no_composition` function, used
 * within parentheses to check for `( expr . identifier )` as member access
 * before function composition claims the dot operator.
 ******************************************************************************/

use crate::{
    ArithmeticOperator, ComparisonOperator, Expression, FunctionComposition, LogicOperator,
    MatchArm, ParseError, Pattern, Program, Term, Token, TypeAnnotation,
};

/*******************************************************************************
 *                              PARSER STRUCT
 *-------------------------------------------------------------------------------
 * `Parser` operates on a token list and a cursor indicating the current token
 * under consideration. The parser steps through the tokens, building the AST
 * if the stream conforms to the grammar, or returning a `ParseError` otherwise.
 ******************************************************************************/
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    //--------------------------------------------------------------------------
    // CONSTRUCTOR
    //--------------------------------------------------------------------------
    /// Creates a new parser given a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    //--------------------------------------------------------------------------
    // parse_program
    //--------------------------------------------------------------------------
    ///
    /// Parses the entire token stream as a single `Program`. Our grammar defines
    /// a program to be just one top-level expression.
    ///
    /// # Errors
    /// Returns a `ParseError` if the tokens do not form a valid expression.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let expression = self.parse_expression()?;
        Ok(Program { expression })
    }

    //--------------------------------------------------------------------------
    // parse_expression
    //--------------------------------------------------------------------------
    ///
    /// Selects the appropriate expression rule:
    ///   * let_expr
    ///   * if_expr
    ///   * lambda
    ///   * pattern_match
    ///   * comparison (with composition attached)
    ///
    /// After parsing a comparison, it calls `parse_composition` to handle
    /// function composition (.) at precedence level 6.
    ///
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Let) => self.parse_let_expr(),
            Some(Token::If) => self.parse_if_expr(),
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Match) => self.parse_pattern_match(),
            _ => {
                // Compare first
                let expr = self.parse_comparison()?;
                // Then apply composition
                self.parse_composition(expr)
            }
        }
    }

    //--------------------------------------------------------------------------
    // parse_expression_no_composition
    //--------------------------------------------------------------------------
    ///
    /// Similar to `parse_expression` but *omits* function composition. This is
    /// used inside parentheses to see if `( expr . identifier )` is a direct
    /// member access rather than composition.
    ///
    fn parse_expression_no_composition(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Let) => self.parse_let_expr(),
            Some(Token::If) => self.parse_if_expr(),
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Match) => self.parse_pattern_match(),
            // stops at comparison
            _ => self.parse_comparison(),
        }
    }

    //--------------------------------------------------------------------------
    // LET EXPRESSION
    //--------------------------------------------------------------------------
    ///
    /// Grammar snippet:
    ///   let_expr = "let" identifier [ ":" type_annotation ] "=" expression "in" expression
    ///
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

    //--------------------------------------------------------------------------
    // IF EXPRESSION
    //--------------------------------------------------------------------------
    ///
    /// if_expr = "if" expression "then" expression "else" expression
    ///
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

    //--------------------------------------------------------------------------
    // LAMBDA
    //--------------------------------------------------------------------------
    ///
    /// lambda = "\" identifier [ ":" type_annotation ] "->" expression
    ///
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

    //--------------------------------------------------------------------------
    // PATTERN MATCH
    //--------------------------------------------------------------------------
    ///
    /// pattern_match = "match" expression "with"
    ///                 "|" pattern "->" expression
    ///                 { "|" pattern "->" expression }
    ///
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

    //--------------------------------------------------------------------------
    // COMPARISON
    //--------------------------------------------------------------------------
    ///
    /// comparison = logic [ ( "==" | "<" | ">" ) logic ]
    ///
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_logic()?;

        if let Some(operator) = match self.current_token() {
            Some(Token::Equal) => Some(ComparisonOperator::Equal),
            Some(Token::LessThan) => Some(ComparisonOperator::LessThan),
            Some(Token::GreaterThan) => Some(ComparisonOperator::GreaterThan),
            _ => None,
        } {
            // consume operator
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

    //--------------------------------------------------------------------------
    // COMPOSITION
    //--------------------------------------------------------------------------
    ///
    /// After comparison, we parse function composition (.) repeatedly, left-associative.
    ///
    fn parse_composition(&mut self, mut left: Expression) -> Result<Expression, ParseError> {
        while let Some(Token::Dot) = self.current_token() {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::FunctionComposition(FunctionComposition {
                f: Box::new(left),
                g: Box::new(right),
            });
        }
        Ok(left)
    }

    //--------------------------------------------------------------------------
    // LOGIC
    //--------------------------------------------------------------------------
    ///
    /// logic = arithmetic [ ( "&&" | "||" ) arithmetic ]
    ///
    fn parse_logic(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_arithmetic()?;

        while let Some(token) = self.current_token() {
            let operator = match token {
                Token::And => LogicOperator::And,
                Token::Or => LogicOperator::Or,
                _ => break,
            };
            self.advance();

            let right = self.parse_arithmetic()?;
            left = Expression::Logic {
                left: Box::new(left),
                operator,
                right: Some(Box::new(right)),
            };
        }
        Ok(left)
    }

    //--------------------------------------------------------------------------
    // ARITHMETIC
    //--------------------------------------------------------------------------
    ///
    /// arithmetic = application { ( "+" | "-" | "*" | "/" ) application }
    ///
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

    //--------------------------------------------------------------------------
    // APPLICATION
    //--------------------------------------------------------------------------
    ///
    /// application = term { term }
    ///
    /// Each extra term in sequence is treated as a function argument to the
    /// preceding expression, forming an `Application` node if multiple are present.
    ///
    fn parse_application(&mut self) -> Result<Expression, ParseError> {
        let mut expressions = vec![self.parse_term()?];

        while let Some(token) = self.current_token() {
            match token {
                Token::Identifier(_)
                | Token::Number(_)
                | Token::LeftParen
                | Token::Wildcard
                | Token::Lambda => {
                    let arg = self.parse_term()?;
                    expressions.push(arg);
                }
                _ => break,
            }
        }

        if expressions.len() > 1 {
            Ok(Expression::Application(expressions))
        } else {
            Ok(expressions.pop().unwrap())
        }
    }

    //--------------------------------------------------------------------------
    // TERM
    //--------------------------------------------------------------------------
    ///
    /// term = identifier
    ///      | number
    ///      | "(" expression ")"
    ///      | "(" expression "." identifier ")"
    ///
    /// This function also integrates logic for optionally parsing a **member access**
    /// of the form `( expr . ident )` by first parsing an expression *without composition*,
    /// then looking ahead for `. identifier )`. If not found, it’s just a grouped expression.
    ///
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            // Identifiers
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                Ok(Expression::Term(Term::Identifier(name_clone)))
            }
            // Numbers
            Some(Token::Number(value)) => {
                let val = *value;
                self.advance();
                Ok(Expression::Term(Term::Number(val)))
            }
            // Parentheses, possibly member access
            Some(Token::LeftParen) => {
                // consume '('
                self.advance();
                let expr = self.parse_expression_no_composition()?;

                // Look for `( expr . identifier )`
                if self.current_token() == Some(&Token::Dot) {
                    if let Some(Token::Identifier(_)) = self.peek_next_token() {
                        if self.peek_two_tokens_ahead() == Some(&Token::RightParen) {
                            // parse member access
                            // consume '.'
                            self.advance();
                            let member_name = match self.current_token() {
                                Some(Token::Identifier(s)) => {
                                    let temp = s.clone();
                                    self.advance();
                                    temp
                                }
                                Some(t) => {
                                    return Err(ParseError::UnexpectedToken {
                                        expected: "identifier".into(),
                                        found: format!("{:?}", t),
                                        message: "Expected identifier after '.' in member access"
                                            .into(),
                                    });
                                }
                                None => return Err(ParseError::UnexpectedEOF),
                            };

                            self.consume_token(
                                Token::RightParen,
                                "Expected ')' after member access",
                            )?;

                            return Ok(Expression::Term(Term::MemberAccess {
                                expression: Box::new(expr),
                                member: member_name,
                            }));
                        }
                    }
                }

                // Otherwise, it’s a grouped expression: ( expr )
                self.consume_token(Token::RightParen, "Expected ')' after expression")?;
                Ok(Expression::Term(Term::GroupedExpression(Box::new(expr))))
            }
            // Lambda can appear as a term
            Some(Token::Lambda) => self.parse_lambda(),

            // Wildcard as a special identifier
            Some(Token::Wildcard) => {
                self.advance();
                Ok(Expression::Term(Term::Identifier("_".into())))
            }

            // Otherwise, error
            Some(t) => Err(ParseError::UnexpectedToken {
                expected: "term".to_string(),
                found: format!("{:?}", t),
                message: "Unexpected token while parsing a term.".into(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    //--------------------------------------------------------------------------
    // PATTERN
    //--------------------------------------------------------------------------
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(s)) => {
                let name = s.clone();
                self.advance();
                Ok(Pattern::Identifier(name))
            }
            Some(Token::Number(n)) => {
                let val = *n;
                self.advance();
                Ok(Pattern::Number(val))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let inner = self.parse_pattern()?;
                self.consume_token(Token::RightParen, "Expected ')' after pattern")?;
                Ok(Pattern::Grouped(Box::new(inner)))
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "pattern".to_string(),
                found: format!("{:?}", token),
                message: "Unexpected token while parsing a pattern.".into(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    //--------------------------------------------------------------------------
    // TYPE ANNOTATION
    //--------------------------------------------------------------------------
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(name)) => {
                let tname = name.clone();
                self.advance();
                match tname.as_str() {
                    "Int" => Ok(TypeAnnotation::Int),
                    "Bool" => Ok(TypeAnnotation::Bool),
                    "String" => Ok(TypeAnnotation::String),
                    "Float" => Ok(TypeAnnotation::Float),
                    "(" => {
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
                    _ => Err(ParseError::InvalidIdentifier(tname)),
                }
            }
            Some(Token::LeftParen) => {
                self.advance();
                let from_type = self.parse_type_annotation()?;
                self.consume_token(Token::Arrow, "Expected '->' in function type")?;
                let to_type = self.parse_type_annotation()?;
                self.consume_token(Token::RightParen, "Expected ')' in function type")?;
                Ok(TypeAnnotation::Function(
                    Box::new(from_type),
                    Box::new(to_type),
                ))
            }
            Some(tok) => Err(ParseError::UnexpectedToken {
                expected: "type annotation".into(),
                found: format!("{:?}", tok),
                message: "Expected a type annotation".into(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    //--------------------------------------------------------------------------
    // TOKEN UTILITY
    //--------------------------------------------------------------------------
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

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        if let Some(Token::Identifier(name)) = self.current_token() {
            let n = name.clone();
            self.advance();
            Ok(n)
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

    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token() == Some(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek_next_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn peek_two_tokens_ahead(&self) -> Option<&Token> {
        self.tokens.get(self.current + 2)
    }
}
