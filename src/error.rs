//! src/error.rs

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// Represents the types of errors that can occur during parsing.
///
/// Parsing errors occur when the input does not conform to the grammar or syntax
/// rules of the language. This enum provides detailed error types for
/// precise diagnostics during parsing.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// An unexpected token was encountered.
    ///
    /// This typically occurs when the parser is expecting one type of token,
    /// but a different token is found.
    UnexpectedToken {
        /// The expected token type or value.
        expected: String,
        /// The actual token encountered.
        found: String,
        /// A descriptive message for additional context.
        message: String,
    },
    /// The parser reached the end of the input unexpectedly.
    ///
    /// This occurs when the input terminates abruptly before a complete
    /// construct (e.g., expression, statement) is parsed.
    UnexpectedEOF,
    /// An invalid number format was encountered.
    ///
    /// This occurs when a numeric literal cannot be parsed into a valid number,
    /// e.g., `1..2` or `1.2.3`.
    InvalidNumberFormat(String),
    /// An invalid identifier was encountered.
    ///
    /// Identifiers must conform to the language's rules for variable and function names.
    InvalidIdentifier(String),
    /// A string literal was not properly terminated.
    ///
    /// This occurs when the input contains a string literal that does not have
    /// a closing quotation mark.
    UnterminatedString,
    /// A pattern match expression is missing arms.
    ///
    /// This occurs when a `match` expression has no branches (arms) to evaluate.
    MissingPatternMatchArm,
    /// A generic or miscellaneous error with a custom message.
    Other(String),
}

//-------------------------------------------------------------------------
// Implementations
//-------------------------------------------------------------------------

impl std::fmt::Display for ParseError {
    /// Provides a user-friendly description of the parsing error.
    ///
    /// This implementation converts each error variant into a descriptive string
    /// suitable for displaying to the user or developer.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "{}: Expected '{}', but found '{}'.",
                    message, expected, found
                )
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of input."),
            ParseError::InvalidNumberFormat(num) => write!(f, "Invalid number format: '{}'.", num),
            ParseError::InvalidIdentifier(id) => write!(f, "Invalid identifier: '{}'.", id),
            ParseError::UnterminatedString => write!(f, "Unterminated string literal."),
            ParseError::MissingPatternMatchArm => {
                write!(f, "Pattern match expression missing arms.")
            }
            ParseError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}
