# RDP (Recursive Descent Parser)

> A minimalist pure functional language implemented via recursive descent parsing.

## Overview

RDP is a small, educational language parser showcasing how to build a **recursive descent parser** for a pure functional language. It highlights the core constructs: **lambda expressions**, **let bindings**, **if-then-else**, **arithmetic**, **function application**, **pattern matching**, and **function composition**.

## Language Features

1. **Pure Functional Semantics**  
   Immutability and first-class functions as fundamental concepts.

2. **Lambda Abstractions**  
   Single-parameter functions using the `\x -> expr` syntax.

3. **Let Bindings**  
   Introduce variables with `let x = ... in ...`.

4. **Function Application**  
   Apply functions to arguments in an expression-oriented style, e.g., `f x`.

5. **Function Composition**  
   Combine functions with the `.` operator, e.g., `(f . g)`.

6. **Basic Arithmetic**  
   Support for `+`, `-`, `*`, `/`.

7. **Conditionals**  
   `if-then-else` expressions for branching logic.

8. **Pattern Matching**  
   `match expr with | pattern -> expr ...` constructs for branching by comparing patterns (identifiers, numbers, grouped).

## Project Layout

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
├── grammar.ebnf
└── src/
    ├── main.rs
    ├── ast.rs
    ├── error.rs
    ├── lib.rs
    ├── lexer.rs
    ├── tokens.rs
    └── parser.rs
```

- **`examples/`**  
  Includes sample `.pfl` files demonstrating language constructs.
- **`grammar.ebnf`**  
  Contains the full EBNF grammar specifying the language’s syntax.
- **`src/`**  
  Source code for the lexer, parser, AST definitions, error handling, and the CLI entry point (`main.rs`).

## Grammar

See [`grammar.ebnf`](grammar.ebnf) for the complete specification of RDP’s syntax. It covers expressions like `let`, `if`, `match`, `lambda`, arithmetic, logical and comparison operators, function application, and function composition.

### Sample Expression

```pfl
let compose = \f -> \g -> \x -> f (g x) in
let double = \x -> x * 2 in
let inc = \x -> x + 1 in
compose double inc 5
```

## Implementation Details

### Lexer

- Converts the input string into a series of tokens: keywords (`let`, `if`, etc.), operators (`+`, `-`, etc.), identifiers, and numbers.

### Parser

- Uses a **recursive descent** approach, matching each grammar rule with a parsing function.
- Produces an **Abstract Syntax Tree (AST)** that mirrors the structure of the language.

### AST

- Models all expressions: `LetExpr`, `IfExpr`, `Lambda`, `PatternMatch`, `Arithmetic`, `Logic`, `Comparison`, `Application`, `Term`, etc.
- Facilitates subsequent interpretation or optimization stages.

## Operator Precedence

RDP enforces the following precedence (highest to lowest):

1. **Parentheses** (`( ... )`)
2. **Function Application** (left-associative)
3. **Function Composition** (`.` operator)
4. **Arithmetic** (`+`, `-`, `*`, `/`)
5. **Comparison** (`==`, `<`, `>`)
6. **Logical** (`&&`, `||`)
7. **Lambda** (`\`)
8. **If-Then-Else**
9. **Let-In**

## Usage

RDP provides a CLI tool to parse `.pfl` files or inline code.

### Requirements

- [Nix](https://determinate.systems/nix-installer/): Recommended for a reproducible dev environment.

### Building

```bash
git clone https://github.com/xosnrdev/rdp.git
cd rdp
nix develop  # optional, to enter the dev shell
cargo build --release
```

### Running the Parser

1. **Parse a File**

   ```bash
   cargo run --release -- examples/arithmetic.pfl
   ```

   Prints the AST to `stdout`.

2. **Parse Source Inline**

   ```bash
   cargo run --release -- "let x = 10 in x + 5"
   ```

3. **Example**

   ```bash
   cargo run --release -- "let compose = \f -> \g -> \x -> f (g x) in compose double inc 5"
   ```

## Example Output

When parsing `let x = 10 in x + 5`, you might see:

```rust
Program {
    expression: LetExpr {
        identifier: "x",
        type_annotation: None,
        value: Term(
            Number(10.0),
        ),
        body: Arithmetic {
            left: Term(
                Identifier("x"),
            ),
            operator: Add,
            right: Term(
                Number(5.0),
            ),
        },
    },
}
```

## Development

### Building, Testing, and Running Examples

```bash
# Build
cargo build

# Run all tests
cargo test

# Parse a sample .pfl file
cargo run --release -- examples/factorial.pfl
```

## Examples

Common demonstrations reside in `examples/`:

- **`arithmetic.pfl`**  
  Basic arithmetic and grouped expressions
- **`compose.pfl`**  
  Composition with the `.` operator
- **`factorial.pfl`**  
  Recursive function example
- **`higher_order.pfl`**  
  Functions receiving other functions
- **`nested_let.pfl`**  
  Nested `let` structures
- **`precedence.pfl`**  
  Operator precedence demonstration

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)  
  Inspiration on building interpreters from scratch.
- [Parsing Techniques](https://dl.acm.org/doi/book/10.5555/1951778)  
  A deep reference for parsing methodologies.

## License

Licensed under [MIT](LICENSE.md). Contributions and forks are welcome!
