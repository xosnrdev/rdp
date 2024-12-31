//! src/ast.rs

/********************************************************************************
 *                           AST (Abstract Syntax Tree)
 *-------------------------------------------------------------------------------*
 * This module contains data structures representing our language’s syntax in
 * a tree form. The parser transforms tokens into these AST nodes, which serve
 * as the foundation for further processing (e.g., interpretation or codegen).
 ********************************************************************************/

/// A complete program is just a single `Expression`. By wrapping it in `Program`,
/// we have a clear entry point for the entire AST.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    /// The root expression of the program.
    pub expression: Expression,
}

/********************************************************************************
 *                          EXPRESSION ENUM
 *-------------------------------------------------------------------------------*
 * The heart of the AST. Each variant represents a distinct language construct,
 * from `let` bindings and lambdas to pattern matches and arithmetic.
 ********************************************************************************/
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A `let` binding (e.g., `let x = ... in ...`).
    LetExpr {
        /// The name bound by this `let`.
        identifier: String,
        /// Optional type annotation (e.g., `x: Int`).
        type_annotation: Option<TypeAnnotation>,
        /// The value assigned to the identifier (right side of `=`).
        value: Box<Expression>,
        /// The body in which the binding is valid (after `in`).
        body: Box<Expression>,
    },

    /// An `if` expression with a condition, `then` branch, and `else` branch.
    IfExpr {
        /// The Boolean condition.
        condition: Box<Expression>,
        /// Evaluated if the condition is true.
        then_branch: Box<Expression>,
        /// Evaluated if the condition is false.
        else_branch: Box<Expression>,
    },

    /// A lambda (anonymous function): `\x -> expr`, possibly with a type annotation.
    Lambda {
        /// The parameter name.
        parameter: String,
        /// The optional type annotation for the parameter.
        type_annotation: Option<TypeAnnotation>,
        /// The lambda body.
        body: Box<Expression>,
    },

    /// A pattern match expression, like `match expr with | pat -> expr | pat -> expr`.
    PatternMatch {
        /// The expression being matched against.
        expression: Box<Expression>,
        /// The arms, each holding a pattern and the corresponding branch expression.
        arms: Vec<MatchArm>,
    },

    /// A comparison (e.g., `x < y`, `a == b`).
    Comparison {
        /// Left-hand side of the comparison.
        left: Box<Expression>,
        /// Comparison operator (`<`, `>`, `==`).
        operator: ComparisonOperator,
        /// The right-hand side (if any). Our grammar supports a single optional comparison.
        right: Option<Box<Expression>>,
    },

    /// A logic operation (e.g., `a && b`, `c || d`).
    Logic {
        /// Left-hand operand.
        left: Box<Expression>,
        /// Logical operator (`&&`, `||`).
        operator: LogicOperator,
        /// The right-hand operand (if present).
        right: Option<Box<Expression>>,
    },

    /// An arithmetic operation like `x + y` or `x * y`.
    Arithmetic {
        /// Left-hand operand.
        left: Box<Expression>,
        /// Arithmetic operator (`+`, `-`, `*`, `/`).
        operator: ArithmeticOperator,
        /// Right-hand operand.
        right: Box<Expression>,
    },

    /// A function or operator application, e.g., `f x y` or `func arg`.
    Application(Vec<Expression>),

    /// A terminal expression (identifier, number, grouped expr, etc.).
    Term(Term),

    /// Function composition node for expressions like `f . g`.
    FunctionComposition(FunctionComposition),
}

/********************************************************************************
 *                                 TERM ENUM
 *-------------------------------------------------------------------------------*
 * Terminal forms in the AST: plain identifiers, numbers, grouped expressions,
 * or member accesses (for expressions in parentheses with a dot).
 ********************************************************************************/
#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    /// A variable or function name.
    Identifier(String),

    /// A numeric literal (floats or ints).
    Number(f64),

    /// A grouped expression, e.g. `(expr)`.
    GroupedExpression(Box<Expression>),

    /// Accessing a member: `(expr).member`.
    MemberAccess {
        expression: Box<Expression>,
        member: String,
    },
}

/********************************************************************************
 *                            PATTERN MATCHING
 *-------------------------------------------------------------------------------*
 * Patterns and arms allow the user to discriminate values of an expression.
 ********************************************************************************/

/// A single `match` arm, pairing a `Pattern` with an expression to evaluate
/// if that pattern matches.
#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub expression: Box<Expression>,
}

/// Patterns recognized in pattern matching, such as identifiers, numbers, or
/// grouped patterns.
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    /// A named pattern (e.g., `x`) or wildcard `_`.
    Identifier(String),

    /// A numeric pattern (e.g., `42`).
    Number(f64),

    /// A grouped pattern `(pat)`.
    Grouped(Box<Pattern>),
}

/********************************************************************************
 *                             TYPE ANNOTATIONS
 *-------------------------------------------------------------------------------*
 * Models our language's type system in the AST, including function types.
 ********************************************************************************/
#[derive(Debug, PartialEq, Clone)]
pub enum TypeAnnotation {
    /// Integer type.
    Int,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// Floating-point type.
    Float,
    /// A function type `(T1 -> T2)`.
    Function(Box<TypeAnnotation>, Box<TypeAnnotation>),
}

/********************************************************************************
 *                              OPERATORS
 *-------------------------------------------------------------------------------*
 * Comparisons, logic, arithmetic, and function composition are each captured
 * in their own small enums or structs.
 ********************************************************************************/

/// Comparison operators (`==`, `<`, `>`).
#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOperator {
    Equal,
    LessThan,
    GreaterThan,
}

/// Logical operators (`&&`, `||`).
#[derive(Debug, PartialEq, Clone)]
pub enum LogicOperator {
    And,
    Or,
}

/// Arithmetic operators (`+`, `-`, `*`, `/`).
#[derive(Debug, PartialEq, Clone)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Represents a function composition operator, typically `.`.
#[derive(Debug, PartialEq, Clone)]
pub enum CompositionOperator {
    Compose,
}

/// A node for function composition `f . g`.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionComposition {
    /// The first function in the composition chain.
    pub f: Box<Expression>,
    /// The second function in the chain.
    pub g: Box<Expression>,
}
