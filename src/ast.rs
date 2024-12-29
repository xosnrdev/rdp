//! src/ast.rs

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------

/// The root node of the Abstract Syntax Tree (AST).
///
/// A program consists of a single expression, which may contain nested expressions.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    /// The root expression of the program.
    pub expression: Expression,
}

/// Represents an expression in the language.
///
/// Expressions form the core of the language's functionality, encompassing
/// constructs like `let` bindings, conditionals, lambdas, pattern matching,
/// comparisons, logic, arithmetic, and more.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A `let` binding, which introduces a new variable or value.
    LetExpr {
        /// The identifier for the binding.
        identifier: String,
        /// An optional type annotation for the binding.
        type_annotation: Option<TypeAnnotation>,
        /// The value to assign to the identifier.
        value: Box<Expression>,
        /// The body where the binding is available.
        body: Box<Expression>,
    },
    /// An `if` expression, representing conditional branching.
    IfExpr {
        /// The condition to evaluate.
        condition: Box<Expression>,
        /// The expression executed if the condition evaluates to true.
        then_branch: Box<Expression>,
        /// The expression executed if the condition evaluates to false.
        else_branch: Box<Expression>,
    },
    /// A lambda (anonymous function) abstraction.
    Lambda {
        /// The parameter name for the lambda.
        parameter: String,
        /// An optional type annotation for the parameter.
        type_annotation: Option<TypeAnnotation>,
        /// The body of the lambda function.
        body: Box<Expression>,
    },
    /// A pattern match expression, allowing branching based on patterns.
    PatternMatch {
        /// The expression to match against.
        expression: Box<Expression>,
        /// The set of match arms to evaluate.
        arms: Vec<MatchArm>,
    },
    /// A comparison expression, e.g., `a < b` or `x == y`.
    Comparison {
        /// The left-hand side of the comparison.
        left: Box<Expression>,
        /// The comparison operator.
        operator: ComparisonOperator,
        /// The optional right-hand side of the comparison.
        right: Option<Box<Expression>>,
    },
    /// A logical operation, e.g., `a && b` or `x || y`.
    Logic {
        /// The left-hand operand.
        left: Box<Expression>,
        /// The logical operator.
        operator: LogicOperator,
        /// The optional right-hand operand.
        right: Option<Box<Expression>>,
    },
    /// An arithmetic operation, e.g., `a + b` or `x * y`.
    Arithmetic {
        /// The left-hand operand.
        left: Box<Expression>,
        /// The arithmetic operator.
        operator: ArithmeticOperator,
        /// The right-hand operand.
        right: Box<Expression>,
    },
    /// A function or operator application, applied to a list of arguments.
    Application(Vec<Expression>),
    /// A terminal node in the AST, such as a literal or grouped expression.
    Term(Term),

    /// Function composition, e.g., `f . g`.
    FunctionComposition(FunctionComposition),
}

/// Represents a terminal expression in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    /// An identifier, e.g., variable or function names.
    Identifier(String),
    /// A numeric literal.
    Number(f64),
    /// A grouped expression, e.g., `(expr)`.
    GroupedExpression(Box<Expression>),
    /// Accessing a member of an object or structure, e.g., `object.member`.
    MemberAccess {
        /// The base expression (e.g., the object or structure).
        expression: Box<Expression>,
        /// The member being accessed.
        member: String,
    },
}

/// Represents a single arm of a pattern match expression.
#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm {
    /// The pattern to match.
    pub pattern: Pattern,
    /// The expression to execute if the pattern matches.
    pub expression: Box<Expression>,
}

/// Represents a pattern in a pattern match expression.
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    /// A pattern matching an identifier.
    Identifier(String),
    /// A pattern matching a numeric literal.
    Number(f64),
    /// A grouped pattern, e.g., `(pattern)`.
    Grouped(Box<Pattern>),
}

/// Represents type annotations for variables, parameters, or expressions.
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
    /// Function type, mapping one type to another.
    Function(Box<TypeAnnotation>, Box<TypeAnnotation>),
}

/// Represents operators for comparisons, e.g., `<`, `>`, `==`.
#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOperator {
    /// Equality operator (`==`).
    Equal,
    /// Less-than operator (`<`).
    LessThan,
    /// Greater-than operator (`>`).
    GreaterThan,
}

/// Represents logical operators, e.g., `&&`, `||`.
#[derive(Debug, PartialEq, Clone)]
pub enum LogicOperator {
    /// Logical AND operator (`&&`).
    And,
    /// Logical OR operator (`||`).
    Or,
}

/// Represents arithmetic operators, e.g., `+`, `-`, `*`, `/`.
#[derive(Debug, PartialEq, Clone)]
pub enum ArithmeticOperator {
    /// Addition operator (`+`).
    Add,
    /// Subtraction operator (`-`).
    Subtract,
    /// Multiplication operator (`*`).
    Multiply,
    /// Division operator (`/`).
    Divide,
}

/// Represents operators for function composition.
#[derive(Debug, PartialEq, Clone)]
pub enum CompositionOperator {
    /// Function composition operator (`.`).
    Compose,
}

/// Represents a function composition expression.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionComposition {
    /// The first function in the composition.
    pub f: Box<Expression>,
    /// The second function in the composition.
    pub g: Box<Expression>,
}
