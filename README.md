# RDP (Recursive Descent Parser)

> A pure functional programming language implementation with recursive descent parsing.

## Overview

This project implements a Recursive Descent Parser for a minimalist pure functional programming language. The implementation focuses on clarity, correctness, and educational value.

## Language Features

- Pure functional semantics
- Lambda abstractions
- Let bindings
- Function application
- Basic arithmetic
- Conditionals

## Project Structure

```
.
├── LICENSE.md
├── README.md
├── examples/
│   └── *.pfl
├── grammar/
│   └── grammar.ebnf
└── src/
    ├── token.rs
    ├── lexer.rs
    └── parser.rs
```

## Grammar

The language follows a formal EBNF grammar supporting functional programming constructs. See [grammar.ebnf](grammar/grammar.ebnf) for the complete specification.

Example expression:

```pfl
let compose = \f -> \g -> \x -> f (g x) in
let double = \x -> x * 2 in
let inc = \x -> x + 1 in
compose double inc 5
```

## Implementation Details

### Parser Features

- Token-based lexical analysis
- Recursive descent parsing
- Error recovery
- Span-based error reporting
- AST generation

### Operator Precedence

1. Parentheses
2. Function application (left associative)
3. Arithmetic operators
4. Comparison operators
5. Lambda abstraction
6. If-then-else
7. Let-in

## Development

Prerequisites:

- Rust 1.81+
- Cargo

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example factorial
```

## Testing

The test suite includes:

- Unit tests for lexer and parser
- Integration tests with example programs
- Error handling tests
- Edge cases validation

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Parsing Techniques](https://dl.acm.org/doi/book/10.5555/1951778)

## License

MIT License - See [LICENSE.md](LICENSE.md) for details
