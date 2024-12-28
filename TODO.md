# TODO

## Tokenizer

- [ ] **Define token types** (e.g., identifiers, literals, operators, delimiters) and their regular expressions.
- [ ] **Implement the tokenizer** to convert source code into tokens, handling whitespace and comments.
- [ ] **Write tests** to verify the tokenizer's functionality.
- [ ] **Support streaming input** and raw byte handling for efficient tokenization from various sources.

## Grammar and Parsing

- [ ] **Analyze the grammar** and fix any left-recursive rules to ensure compatibility with top-down parsing.
- [ ] **Design the top-down parser** and implement parsing functions for different grammar rules.
- [ ] **Add lookahead** for predictive parsing to handle ambiguous grammars and make parsing decisions.
- [ ] **Implement error handling** to gracefully manage unexpected tokens and syntax errors.
- [ ] **Implement function composition operator (`.`)** and **unary operators (`-`, `!`)** to enhance expression capabilities.
- [ ] **Refine operator precedence and associativity** to ensure correct parsing of complex expressions.
- [ ] **Implement error recovery mechanisms** to allow the parser to continue after encountering errors, enabling the detection of multiple issues in a single run.

## Abstract Syntax Tree (AST)

- [ x ] **Design AST node structures** to accurately represent the hierarchical nature of the source code.
- [ ] **Implement the AST builder** within the parser to construct the AST during parsing.
- [ ] **Create tests** to ensure the AST accurately represents the source code's structure and semantics.

## Parser Integration

- [ ] **Integrate the tokenizer and parser** to work seamlessly together, enabling the end-to-end parsing of source code.
- [ ] **Support complex features** like functions, control flow constructs, and pattern matching to enhance the language's expressiveness.
- [ ] **Optimize the parser for performance**, ensuring efficient parsing even for large and complex source files.

## Error Handling

- [ ] **Implement comprehensive error reporting** with detailed messages, including line and column numbers, to aid in debugging and development.
- [ ] **Enhance error messages** to be clear and actionable, guiding users to resolve syntax issues effectively.

## Testing

- [ ] **Write unit tests for the lexer** to ensure accurate tokenization across various input scenarios.
- [ ] **Write unit tests for the parser** to verify correct AST generation and parsing logic.
- [ ] **Write property-based tests for the parser** using `proptest` to explore a wide range of input cases and ensure robustness against edge cases.
- [ ] **Implement tests for error scenarios**, ensuring that the parser correctly identifies and reports syntax errors.

## Documentation

- [ ] **Add comprehensive documentation** with doc comments (`///`) for modules, structures, enums, and functions to facilitate maintainability and ease of understanding.
- [ ] **Provide usage examples** in the documentation to demonstrate how to utilize the parser effectively.
- [ ] **Generate HTML documentation** using `cargo doc --open` to make the documentation easily accessible.
- [ ] **Maintain up-to-date documentation** reflecting all changes and enhancements made to the parser.
