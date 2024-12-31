//! src/tokens.rs

/********************************************************************************
 *                              TOKEN MODULE
 *-------------------------------------------------------------------------------*
 * This module defines the set of tokens recognized by our language lexer.
 * Each token corresponds to the smallest atomic unit of the syntax, such as
 * keywords, operators, delimiters, identifiers, or numeric literals.
 *
 * By enumerating all possible tokens here, both the lexer (which emits tokens)
 * and the parser (which consumes tokens) share a clear contract of allowed
 * symbols.
 ********************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    //--------------------------------------------------------------------------
    // Keywords
    //--------------------------------------------------------------------------
    /// Represents the `let` keyword used in binding expressions.
    Let,

    /// Represents the `in` keyword, often paired with `let`.
    In,

    /// Represents the `if` keyword for conditional expressions.
    If,

    /// Represents the `then` keyword, part of an if-expression structure.
    Then,

    /// Represents the `else` keyword, completing an if-expression.
    Else,

    /// Represents the `match` keyword for pattern matching.
    Match,

    /// Represents the `with` keyword, used with match-expressions.
    With,

    /// Represents the `\` symbol for lambda abstractions.
    Lambda,

    //--------------------------------------------------------------------------
    // Operators
    //--------------------------------------------------------------------------
    /// Equality operator (`==`).
    Equal,

    /// Less-than operator (`<`).
    LessThan,

    /// Greater-than operator (`>`).
    GreaterThan,

    /// Logical AND operator (`&&`).
    And,

    /// Logical OR operator (`||`).
    Or,

    /// Plus operator (`+`).
    Plus,

    /// Minus operator (`-`).
    Minus,

    /// Multiplication operator (`*`).
    Star,

    /// Division operator (`/`).
    Slash,

    /// Arrow operator (`->`), used in function types and lambdas.
    Arrow,

    /// Dot operator (`.`), relevant for composition or member access.
    Dot,

    /// Pipe symbol (`|`), often used in pattern matching arms.
    Pipe,

    //--------------------------------------------------------------------------
    // Literals
    //--------------------------------------------------------------------------
    /// Identifiers, e.g., variable or function names.
    Identifier(String),

    /// Numeric literal, storing a floating-point value for both int and float.
    Number(f64),

    //--------------------------------------------------------------------------
    // Delimiters
    //--------------------------------------------------------------------------
    /// Left parenthesis (`(`).
    LeftParen,

    /// Right parenthesis (`)`).
    RightParen,

    /// Colon (`:`), often used for type annotations.
    Colon,

    /// Assignment operator (`=`), used in bindings (`let x = expr`).
    Assign,

    //--------------------------------------------------------------------------
    // Wildcard
    //--------------------------------------------------------------------------
    /// Wildcard identifier (`_`), commonly used in patterns.
    Wildcard,

    //--------------------------------------------------------------------------
    // End of File
    //--------------------------------------------------------------------------
    /// End-of-file marker. Indicates no more tokens are available.
    Eof,
}
