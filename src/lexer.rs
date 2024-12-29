//! src/lexer.rs

use crate::{ParseError, Token};

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// The `Lexer` is responsible for tokenizing the input source code.
///
/// Tokenization is the process of breaking a raw input string into a sequence
/// of tokens that represent the smallest syntactic units of the language.
/// This is the first stage of parsing and ensures that the input is prepared
/// for syntax analysis.
pub struct Lexer {
    /// The input source code, split into individual characters.
    input: Vec<char>,
    /// The current position in the input.
    current: usize,
}

//-------------------------------------------------------------------------
// Implementations
//-------------------------------------------------------------------------

impl Lexer {
    /// Creates a new `Lexer` instance from a given input string.
    ///
    /// # Arguments
    /// - `input`: The source code to be tokenized.
    ///
    /// # Returns
    /// A new `Lexer` instance ready to tokenize the input.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            current: 0,
        }
    }

    /// Lexes the entire input string into a vector of tokens.
    ///
    /// This method processes the input source code, identifying all tokens
    /// and handling errors where the input is malformed.
    ///
    /// # Returns
    /// - `Ok(Vec<Token>)` if tokenization is successful.
    /// - `Err(ParseError)` if an error occurs during tokenization.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::Eof); // Append the end-of-file token
        Ok(tokens)
    }

    /// Reads the next token from the input source code.
    ///
    /// This method identifies the next meaningful token in the input, skipping
    /// over whitespace and handling various token types (e.g., keywords, operators,
    /// numbers, and identifiers).
    ///
    /// # Returns
    /// - `Ok(Token)` if a valid token is identified.
    /// - `Err(ParseError)` if an unexpected or invalid input is encountered.
    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::Eof);
        }

        let c = self.advance();

        match c {
            // Keywords and symbols
            'l' if self.peek_keyword("et") => self.consume_keyword("et", Token::Let),
            'i' if self.peek_keyword("f") => self.consume_keyword("f", Token::If),
            't' if self.peek_keyword("hen") => self.consume_keyword("hen", Token::Then),
            'e' if self.peek_keyword("lse") => self.consume_keyword("lse", Token::Else),
            'm' if self.peek_keyword("atch") => self.consume_keyword("atch", Token::Match),
            'w' if self.peek_keyword("ith") => self.consume_keyword("ith", Token::With),
            '\\' => Ok(Token::Lambda),
            '=' if self.match_char('=') => Ok(Token::Equal),
            '<' => Ok(Token::LessThan),
            '>' => Ok(Token::GreaterThan),
            '&' if self.match_char('&') => Ok(Token::And),
            '|' if self.match_char('|') => Ok(Token::Or),
            '+' => Ok(Token::Plus),
            '-' if self.match_char('>') => Ok(Token::Arrow),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Wildcard),
            '/' => Ok(Token::Slash),
            '.' => Ok(Token::Dot),
            '|' => Ok(Token::Pipe),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            ':' => Ok(Token::Colon),
            '=' => Ok(Token::Assign),

            // Numbers
            c if c.is_ascii_digit() => self.number(c),

            // Identifiers
            c if c.is_ascii_alphabetic() => self.identifier(c),

            // Wildcard identifier
            '_' => Ok(Token::Identifier("_".to_string())),

            // Unexpected character
            _ => Err(ParseError::UnexpectedToken {
                expected: "valid token".to_string(),
                found: c.to_string(),
                message: "Unexpected character".to_string(),
            }),
        }
    }

    /// Consumes a numeric literal token.
    ///
    /// Supports integers and floating-point numbers.
    ///
    /// # Arguments
    /// - `start`: The first digit of the number.
    ///
    /// # Returns
    /// - `Ok(Token::Number)` if the number is valid.
    /// - `Err(ParseError)` if the number format is invalid.
    fn number(&mut self, start: char) -> Result<Token, ParseError> {
        let mut value = start.to_string();

        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            value.push(self.advance());
        }

        if self.peek() == Some('.') {
            value.push(self.advance());
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                value.push(self.advance());
            }
        }

        value
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| ParseError::InvalidNumberFormat(value))
    }

    /// Consumes an identifier or keyword token.
    ///
    /// # Arguments
    /// - `start`: The first character of the identifier or keyword.
    ///
    /// # Returns
    /// - `Ok(Token)` representing either a keyword or an identifier.
    fn identifier(&mut self, start: char) -> Result<Token, ParseError> {
        let mut value = start.to_string();

        while self.peek().map_or(false, |c| c.is_ascii_alphanumeric()) {
            value.push(self.advance());
        }

        // Check for keywords
        match value.as_str() {
            "let" => Ok(Token::Let),
            "in" => Ok(Token::In),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "match" => Ok(Token::Match),
            "with" => Ok(Token::With),
            // Default to identifier
            _ => Ok(Token::Identifier(value)),
        }
    }

    /// Skips over whitespace characters in the input.
    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }
    }

    /// Peeks at the next character without consuming it.
    ///
    /// # Returns
    /// The next character, if available.
    fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }

    /// Checks if the current sequence matches a keyword.
    ///
    /// # Arguments
    /// - `keyword`: The keyword to check for.
    ///
    /// # Returns
    /// `true` if the keyword matches, `false` otherwise.
    fn peek_keyword(&self, keyword: &str) -> bool {
        self.input[self.current..]
            .iter()
            .take(keyword.len())
            .collect::<String>()
            == keyword
    }

    /// Consumes a keyword if it matches the current input.
    ///
    /// # Arguments
    /// - `keyword`: The keyword to consume.
    /// - `token`: The token to return if the keyword matches.
    ///
    /// # Returns
    /// `Ok(Token)` representing the matched keyword.
    fn consume_keyword(&mut self, keyword: &str, token: Token) -> Result<Token, ParseError> {
        for _ in 0..keyword.len() {
            self.advance();
        }
        Ok(token)
    }

    /// Advances the lexer and consumes the current character.
    ///
    /// # Returns
    /// The consumed character.
    fn advance(&mut self) -> char {
        let c = self.input[self.current];
        self.current += 1;
        c
    }

    /// Matches and consumes a specific character if it is the next in the input.
    ///
    /// # Arguments
    /// - `expected`: The character to match.
    ///
    /// # Returns
    /// `true` if the character is matched, `false` otherwise.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Checks if the lexer has reached the end of the input.
    ///
    /// # Returns
    /// `true` if at the end of input, `false` otherwise.
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
