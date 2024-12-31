//! src/error.rs

/****************************************************************************
 *                               ERROR MODULE
 *--------------------------------------------------------------------------
 * Defines `ParseError` and related functionality. These errors surface when
 * the parser encounters invalid tokens, mismatched structures, or abrupt ends
 * in the input. Each variant provides descriptive information to guide
 * debugging and error reporting.
 ****************************************************************************/

use std::{error, fmt};

/// Enumerates all parse errors that may appear when tokenizing or parsing.
///
/// Each variant holds enough context for downstream systems to identify
/// where and why parsing failed.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// Signifies that the parser encountered a token other than what
    /// was expected. Contains details on what was expected, what was found,
    /// and a short message describing the context.
    UnexpectedToken {
        expected: String,
        found: String,
        message: String,
    },

    /// Indicates an abrupt end of input before a complete construct could
    /// be parsed.
    UnexpectedEOF,

    /// Raised when a numeric literal doesn’t parse cleanly (e.g., `12.3.4`).
    InvalidNumberFormat(String),

    /// Raised when an identifier doesn’t conform to the language’s naming rules.
    InvalidIdentifier(String),

    /// Raised when the lexer finds a string literal that never terminates.
    UnterminatedString,

    /// Signifies that a `match` expression has no pattern arms.
    MissingPatternMatchArm,

    /// A catch-all for errors that don’t fit other variants.
    Other(String),
}

impl fmt::Display for ParseError {
    /// Renders an error variant into a user-friendly string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "{}: expected '{}', but found '{}'.",
                    message, expected, found
                )
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file."),
            ParseError::InvalidNumberFormat(num) => {
                write!(f, "Invalid number format: '{}'.", num)
            }
            ParseError::InvalidIdentifier(id) => {
                write!(f, "Invalid identifier: '{}'.", id)
            }
            ParseError::UnterminatedString => write!(f, "Unterminated string literal."),
            ParseError::MissingPatternMatchArm => {
                write!(f, "Pattern match expression missing arms.")
            }
            ParseError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl error::Error for ParseError {}
