# TODO

- [ ] **Implement Term with Member Access**

  - Parse expressions like `(object.member)`
  - Handle nested member accesses, e.g., `((a.b).c)`
  - Update AST to represent member access correctly

- [ ] **Implement Function Composition**

  - Parse function compositions using the `.` operator, e.g., `(f . g)`
  - Define operator precedence and associativity for `.`
  - Update AST to include function composition nodes

- [ ] **Handle the `.` Operator in Lexer and Parser**

  - Ensure `.` is tokenized as `Token::Dot`
  - Distinguish between `.` used for member access and function composition based on context
  - Update parsing rules accordingly

- [ ] **Implement Comment Handling**

  - Support single-line comments using `//`
  - Optionally add multi-line comment support with `/* ... */`
  - Ensure comments are skipped during tokenization and don't affect parsing

- [ ] **Implement Higher-Order Functions**

  - Enable parsing of lambda expressions with multiple parameters, e.g., `\f, x -> ...`
  - Update lexer to recognize and tokenize commas (`,`) as `Token::Comma`
  - Modify parser to handle multiple parameters in function definitions
  - Update AST to represent functions with multiple parameters
  - Add test cases for higher-order functions

- [ ] **Update and Run Tests**

  - Add test cases for member access and function composition
  - Include tests with comments to verify proper handling
  - Add test cases for higher-order functions, ensuring multiple parameters are parsed correctly
  - Ensure all existing and new tests pass successfully

- [ ] **Refactor Parser for Maintainability**

  - Clean up and organize parsing functions
  - Optimize code for better readability and performance

- [ ] **Document New Features**
  - Update project documentation to include member access and function composition
  - Provide examples demonstrating the new features
  - Document the handling of higher-order functions and multiple lambda parameters
