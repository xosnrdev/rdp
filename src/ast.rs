//! src/ast.rs

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// Represents the entire program.
#[derive(Debug, PartialEq)]
pub enum Program {
    /// The program consists of a single expression.
    Expression(Box<Expression>),
}

/// Enumerates all possible expressions in the language.
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// A let-binding expression.
    Let {
        /// The identifier being bound.
        identifier: String,
        /// Optional type annotation.
        type_annotation: Option<TypeAnnotation>,
        /// The value being bound to the identifier.
        value: Box<Expression>,
        /// The body expression where the identifier is in scope.
        body: Box<Expression>,
    },
    /// An if-then-else expression.
    If {
        /// The condition expression.
        condition: Box<Expression>,
        /// The expression executed if the condition is true.
        then_branch: Box<Expression>,
        /// The expression executed if the condition is false.
        else_branch: Box<Expression>,
    },
    /// A lambda (anonymous function) expression.
    Lambda {
        /// The parameter name.
        parameter: String,
        /// Optional type annotation for the parameter.
        type_annotation: Option<TypeAnnotation>,
        /// The body of the lambda.
        body: Box<Expression>,
    },
    /// A pattern matching expression.
    Match {
        /// The expression being matched.
        expr: Box<Expression>,
        /// A list of pattern-expression pairs.
        arms: Vec<(Pattern, Box<Expression>)>,
    },
    /// A comparison expression (e.g., `x == y`).
    Comparison {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The comparison operator.
        operator: ComparisonOp,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
    /// A logical expression (e.g., `a && b`).
    Logic {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The logical operator.
        operator: LogicOp,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
    /// An arithmetic expression (e.g., `a + b`).
    Arithmetic {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The arithmetic operator.
        operator: ArithmeticOp,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
    /// A function application expression (e.g., `f x`).
    Application {
        /// The function being applied.
        function: Box<Expression>,
        /// The list of argument expressions.
        arguments: Vec<Box<Expression>>,
    },
    /// A function composition expression (e.g., `f . g`).
    FunctionComposition {
        /// The left function.
        left: Box<Expression>,
        /// The right function.
        right: Box<Expression>,
    },
    /// A unary operation expression (e.g., `-x`).
    Unary {
        /// The unary operator.
        operator: UnaryOp,
        /// The operand expression.
        operand: Box<Expression>,
    },
    /// A term, which is a basic unit in expressions.
    Term(Term),
}

/// Enumerates the basic terms in expressions.
#[derive(Debug, PartialEq)]
pub enum Term {
    /// An identifier.
    Identifier(String),
    /// A numeric literal.
    Number(f64),
    /// A grouped expression.
    Grouping(Box<Expression>),
    /// A field access (e.g., `object.method`).
    FieldAccess(Box<Expression>, String),
}

/// Enumerates patterns used in pattern matching.
#[derive(Debug, PartialEq)]
pub enum Pattern {
    /// An identifier pattern.
    Identifier(String),
    /// A numeric literal pattern.
    Number(f64),
    /// A grouped pattern.
    Grouping(Box<Pattern>),
}

/// Enumerates type annotations.
#[derive(Debug, PartialEq)]
pub enum TypeAnnotation {
    /// Integer type.
    Int,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// Floating-point type.
    Float,
    /// Function type (e.g., `(Int -> Bool)`).
    Function(Box<TypeAnnotation>, Box<TypeAnnotation>),
}

/// Enumerates comparison operators.
#[derive(Debug, PartialEq)]
pub enum ComparisonOp {
    /// Equality operator (`==`).
    Equal,
    /// Less-than operator (`<`).
    LessThan,
    /// Greater-than operator (`>`).
    GreaterThan,
}

/// Enumerates logical operators.
#[derive(Debug, PartialEq)]
pub enum LogicOp {
    /// Logical AND operator (`&&`).
    And,
    /// Logical OR operator (`||`).
    Or,
}

/// Enumerates arithmetic operators.
#[derive(Debug, PartialEq)]
pub enum ArithmeticOp {
    /// Addition operator (`+`).
    Add,
    /// Subtraction operator (`-`).
    Subtract,
    /// Multiplication operator (`*`).
    Multiply,
    /// Division operator (`/`).
    Divide,
}

/// Enumerates unary operators.
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    /// Negation operator (`-`).
    Negate,
    /// Logical NOT operator (`!`).
    Not,
}
