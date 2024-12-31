# RDP (Recursive Descent Parser)

> A pure functional programming language implementation with recursive descent parsing.

## Overview

RDP is a Recursive Descent Parser crafted for a minimalist pure functional programming language. Designed with clarity and educational purposes in mind, RDP emphasizes straightforward implementation of parsing techniques while supporting essential functional programming constructs.

## Language Features

RDP supports the following features:

- **Pure Functional Semantics**: Emphasizes immutability and first-class functions.
- **Lambda Abstractions**: Anonymous functions using the `\` syntax.
- **Let Bindings**: Variable declarations and bindings.
- **Function Application**: Applying functions to arguments.
- **Function Composition**: Combining functions using the `.` operator.
- **Basic Arithmetic**: Support for addition, subtraction, multiplication, and division.
- **Conditionals**: `if-then-else` expressions.
- **Pattern Matching**: Deconstructing data structures with `match`.
- **Comments**: Single-line (`//`) and multi-line (`/* ... */`) comments.

## Project Structure

```
.
├── LICENSE.md
├── README.md
├── examples/
│   ├── arithmetic.pfl
│   ├── compose.pfl
│   ├── factorial.pfl
│   ├── higher_order.pfl
│   ├── nested_let.pfl
│   └── precedence.pfl
├── grammar/
│   └── grammar.ebnf
└── src/
    ├── main.rs
    ├── ast.rs
    ├── error.rs
    ├── lib.rs
    ├── lexer.rs
    ├── tokens.rs
    └── parser.rs
```

- **`examples/`**: Contains example `.pfl` files demonstrating various language features.
- **`grammar/grammar.ebnf`**: Defines the language's grammar in EBNF format.
- **`src/`**: Source code for the lexer, parser, AST definitions, and the main executable.

## Grammar

RDP's language is defined by an EBNF grammar that outlines the syntax and structure of valid programs. The grammar supports essential functional programming constructs.

See [grammar.ebnf](grammar.ebnf) for the complete specification.

### Example Expression

```pfl
let compose = \f -> \g -> \x -> f (g x) in
let double = \x -> x * 2 in
let inc = \x -> x + 1 in
compose double inc 5
```

## Implementation Details

### Components

- **Lexer**: Converts source code into tokens, handling syntax elements like identifiers, numbers, operators, and comments.
- **Parser**: Processes tokens to build an Abstract Syntax Tree (AST) based on the defined grammar.
- **AST**: Represents the syntactic structure of the program, facilitating further processing like interpretation or compilation.

### Parser Features

- **Token-Based Lexical Analysis**: Efficiently tokenizes the input source.
- **Recursive Descent Parsing**: Implements parsing functions corresponding to grammar rules.
- **Error Recovery**: Attempts to continue parsing after encountering errors to collect multiple errors in a single run.
- **Span-Based Error Reporting**: Provides precise locations (line and column) for syntax errors.
- **AST Generation**: Constructs a hierarchical AST representing the program structure.

### Operator Precedence

The parser respects the following operator precedence, from highest to lowest:

1. **Parentheses**: `(` `)`
2. **Function Application**: Left-associative
3. **Function Composition**: Left-associative (`.` operator)
4. **Arithmetic Operators**: `*`, `/`, `+`, `-`
5. **Comparison Operators**: `==`, `<`, `>`
6. **Logical Operators**: `&&`, `||`
7. **Lambda Abstraction**: `\`
8. **If-Then-Else**
9. **Let-In**

## Usage

RDP functions as a command-line interface (CLI), allowing you to parse `.pfl` files or provide source code directly.

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust from [rustup.rs](https://rustup.rs/).
- **Cargo**: Rust's package manager, installed alongside Rust.

### Building the Project

Clone the repository and navigate to its directory:

```bash
git clone https://github.com/xosnrdev/rdp.git
cd rdp
```

Build the project using Cargo:

```bash
cargo build --release
```

### Running the Parser

#### Parsing a File

Provide a `.pfl` file as an argument:

```bash
cargo run --release -- examples/arithmetic.pfl
```

#### Parsing Direct Source Code

Provide source code directly as a command-line argument:

```bash
cargo run --release -- "let x = 10 in x + 5"
```

#### Example Command

```bash
cargo run --release -- "let compose = \f -> \g -> \x -> f (g x) in compose double inc 5"
```

### Example Output

The parser outputs the Abstract Syntax Tree (AST) in a pretty-printed format.

```rust
Program {
    expression: LetExpr {
        identifier: "x",
        type_annotation: None,
        value: Term(
            Number(
                10.0,
            ),
        ),
        body: Arithmetic {
            left: Term(
                Identifier(
                    "x",
                ),
            ),
            operator: Add,
            right: Term(
                Number(
                    5.0,
                ),
            ),
        },
    },
}
```

## Development

### Building and Running

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run a specific example
cargo run --release -- examples/factorial.pfl
```

### Testing

The test suite includes:

- **Unit Tests**: For lexer and parser components.
- **Integration Tests**: Using example `.pfl` files.
- **Error Handling Tests**: Ensuring robust error reporting.
- **Edge Cases**: Validating parser behavior with complex and unusual inputs.

Run all tests with:

```bash
cargo test
```

## Examples

The `examples/` directory contains various `.pfl` files demonstrating different language features:

- **`arithmetic.pfl`**: Basic arithmetic operations and grouped expressions.
- **`compose.pfl`**: Function composition using the `.` operator.
- **`factorial.pfl`**: Recursive function implementation for calculating factorial.
- **`higher_order.pfl`**: Higher-order functions, passing functions as arguments.
- **`nested_let.pfl`**: Nested `let` expressions and variable shadowing.
- **`precedence.pfl`**: Operator precedence and associativity.
- **`comment_test.pfl`**: Usage of comments within code.

### Sample Example: `higher_order.pfl`

```pfl
// higher_order.pfl

let apply_twice = \f, x -> f (f x) in
apply_twice increment 5
```

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Parsing Techniques](https://dl.acm.org/doi/book/10.5555/1951778)

## License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.
