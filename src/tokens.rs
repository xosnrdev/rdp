//! src/tokens.rs

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// Defines the set of tokens used by the parser.
///
/// Tokens represent the smallest units of meaning in the language's syntax.
/// These tokens will be produced during the lexical analysis (tokenization)
/// phase and consumed during parsing.
///
/// # Variants
/// - Keywords such as `Let` or `If`
/// - Operators like `+`, `-`, or `&&`
/// - Literals such as identifiers and numbers
/// - Delimiters such as parentheses or colons
/// - End-of-file marker for indicating the end of the input
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    /// Represents the `let` keyword.
    Let,
    /// Represents the `in` keyword.
    In,
    /// Represents the `if` keyword.
    If,
    /// Represents the `then` keyword.
    Then,
    /// Represents the `else` keyword.
    Else,
    /// Represents the `match` keyword.
    Match,
    /// Represents the `with` keyword.
    With,
    /// Represents a lambda abstraction, denoted by `\` in the source code.
    Lambda,

    // Operators
    /// Represents the equality operator (`==`).
    Equal,
    /// Represents the less-than operator (`<`).
    LessThan,
    /// Represents the greater-than operator (`>`).
    GreaterThan,
    /// Represents the logical AND operator (`&&`).
    And,
    /// Represents the logical OR operator (`||`).
    Or,
    /// Represents the addition operator (`+`).
    Plus,
    /// Represents the subtraction operator (`-`).
    Minus,
    /// Represents the multiplication operator (`*`).
    Wildcard,
    /// Represents the division operator (`/`).
    Slash,
    /// Represents the arrow operator (`->`), often used for function types.
    Arrow,
    /// Represents a period (`.`), used in member access or other syntactic constructs.
    Dot,
    /// Represents a pipe (`|`), used in pattern matching or function composition.
    Pipe,

    // Literals
    /// Represents an identifier, e.g., variable or function names.
    Identifier(String),
    /// Represents a numeric literal.
    Number(f64),

    // Delimiters
    /// Represents the left parenthesis (`(`).
    LeftParen,
    /// Represents the right parenthesis (`)`).
    RightParen,
    /// Represents a colon (`:`), often used in type annotations or key-value pairs.
    Colon,
    /// Represents the assignment operator (`=`) used in variable declarations like `let x = ...`.
    Assign,

    // End of File
    /// Represents the end of the input source, signaling no more tokens.
    Eof,
}
