//! src/lexer.rs

/********************************************************************************
 *                            LEXER MODULE
 *-------------------------------------------------------------------------------*
 * This module converts raw input text into tokens defined in `token.rs`. It
 * scans the input character-by-character, categorizing sequences into keywords,
 * identifiers, numeric literals, operators, and more. The parser later uses
 * these tokens for syntax analysis.
 ********************************************************************************/

use crate::{ParseError, Token};

/*-----------------------------------------------------------------------------
 *                              LEXER STRUCT
 *-----------------------------------------------------------------------------
 * The `Lexer` holds the input as a vector of characters (`input`) and a cursor
 * index (`current`). Methods on the `Lexer` advance through the input, producing
 * tokens until exhaustion or error.
 *---------------------------------------------------------------------------*/
pub struct Lexer {
    /// The entire input, split into characters.
    input: Vec<char>,

    /// Current position in `input`.
    current: usize,
}

impl Lexer {
    //--------------------------------------------------------------------------
    // CONSTRUCTOR
    //--------------------------------------------------------------------------

    /// Creates a new `Lexer` from a &str. Internally stores the string’s characters.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
        }
    }

    //--------------------------------------------------------------------------
    // PUBLIC API
    //--------------------------------------------------------------------------

    /// Converts the entire input into a vector of `Token`s.
    ///
    /// This processes each chunk of text until we reach the end, returning
    /// `Ok(Vec<Token>)` on success, or `Err(ParseError)` if tokenization fails
    /// due to malformed input.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        // Keep producing tokens until we exhaust the input.
        while !self.is_at_end() {
            let token = self.next_token()?;
            tokens.push(token);
        }

        // Append EOF marker.
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    //--------------------------------------------------------------------------
    // NEXT TOKEN
    //--------------------------------------------------------------------------

    /// Fetches the next meaningful token, skipping any whitespace encountered.
    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        // If we’re at end, return EOF token.
        if self.is_at_end() {
            return Ok(Token::Eof);
        }

        // Advance and examine the next character.
        let c = self.advance();

        match c {
            // Check for keyword starts: e.g. 'l' -> "let", 'm' -> "match".
            'l' if self.peek_keyword("et") => self.consume_keyword("et", Token::Let),
            'i' if self.peek_keyword("f") => self.consume_keyword("f", Token::If),
            't' if self.peek_keyword("hen") => self.consume_keyword("hen", Token::Then),
            'e' if self.peek_keyword("lse") => self.consume_keyword("lse", Token::Else),
            'm' if self.peek_keyword("atch") => self.consume_keyword("atch", Token::Match),
            'w' if self.peek_keyword("ith") => self.consume_keyword("ith", Token::With),

            // Single-char or small multi-char operators.
            '\\' => Ok(Token::Lambda),
            '=' if self.match_char('=') => Ok(Token::Equal),
            '<' => Ok(Token::LessThan),
            '>' => Ok(Token::GreaterThan),
            '&' if self.match_char('&') => Ok(Token::And),
            '|' if self.match_char('|') => Ok(Token::Or),
            '+' => Ok(Token::Plus),
            '-' if self.match_char('>') => Ok(Token::Arrow),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            '.' => Ok(Token::Dot),
            '|' => Ok(Token::Pipe),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            ':' => Ok(Token::Colon),
            '=' => Ok(Token::Assign),

            // If the character is numeric, parse a number literal.
            ch if ch.is_ascii_digit() => self.number(ch),

            // If the character is alphabetic, parse an identifier (or potential keyword).
            ch if ch.is_ascii_alphabetic() => self.identifier(ch),

            // Underscore is recognized as a wildcard pattern.
            '_' => Ok(Token::Wildcard),

            // Anything else is invalid or unexpected.
            _ => Err(ParseError::UnexpectedToken {
                expected: "valid token".to_string(),
                found: c.to_string(),
                message: "Unexpected character".to_string(),
            }),
        }
    }

    //--------------------------------------------------------------------------
    // NUMBER LITERALS
    //--------------------------------------------------------------------------

    /// Parses a numeric literal (integer or floating-point).
    ///
    /// # Arguments
    /// * `start` - the initial digit we encountered.
    fn number(&mut self, start: char) -> Result<Token, ParseError> {
        let mut value = start.to_string();

        // Accumulate any additional digits.
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            value.push(self.advance());
        }

        // If the next character is '.', collect decimal digits.
        if self.peek() == Some('.') {
            value.push(self.advance());

            // Gather any digits after the decimal point.
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                value.push(self.advance());
            }
        }

        // Convert to a floating-point value, or raise an error if invalid.
        value
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| ParseError::InvalidNumberFormat(value))
    }

    //--------------------------------------------------------------------------
    // IDENTIFIERS OR KEYWORDS
    //--------------------------------------------------------------------------

    /// Parses an identifier or falls back to a keyword if `value` matches one.
    ///
    /// # Arguments
    /// * `start` - the initial alphabetic character.
    fn identifier(&mut self, start: char) -> Result<Token, ParseError> {
        let mut text = start.to_string();

        // Accumulate subsequent alphanumeric chars.
        while self.peek().map_or(false, |c| c.is_ascii_alphanumeric()) {
            text.push(self.advance());
        }

        // Check if it’s one of our known keywords (like "in"). Otherwise, an identifier.
        match text.as_str() {
            "let" => Ok(Token::Let),
            "in" => Ok(Token::In),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "match" => Ok(Token::Match),
            "with" => Ok(Token::With),
            _ => Ok(Token::Identifier(text)),
        }
    }

    //--------------------------------------------------------------------------
    // WHITESPACE SKIPPING
    //--------------------------------------------------------------------------

    /// Discards any leading whitespace before identifying a token.
    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }
    }

    //--------------------------------------------------------------------------
    // STRING VIEW & KEYWORD CHECK
    //--------------------------------------------------------------------------

    /// Verifies if the upcoming slice of input matches a given `keyword`.
    /// (Used for multi-char checks like "et", "hen", etc.)
    fn peek_keyword(&self, keyword: &str) -> bool {
        self.input[self.current..]
            .iter()
            .take(keyword.len())
            .collect::<String>()
            == keyword
    }

    /// Consumes a keyword from the input if it matches, returning the `token`.
    fn consume_keyword(&mut self, keyword: &str, token: Token) -> Result<Token, ParseError> {
        // We already confirmed the match; just advance past the keyword.
        for _ in 0..keyword.len() {
            self.advance();
        }
        Ok(token)
    }

    //--------------------------------------------------------------------------
    // CHARACTER UTILITIES
    //--------------------------------------------------------------------------

    /// Consumes and returns the next character in `input`.
    fn advance(&mut self) -> char {
        let ch = self.input[self.current];
        self.current += 1;
        ch
    }

    /// If the next character matches `expected`, consume it. Otherwise, return false.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Provides the next character without consuming it, if available.
    fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }

    /// Checks whether we have reached or passed the end of the input.
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
