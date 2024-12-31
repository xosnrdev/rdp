# TODO

- [x] **Implement Term with Member Access**

  - Parse expressions like `(object.member)`
  - Handle nested member accesses, e.g. `((a.b).c)`
  - Update AST to represent member access correctly

- [x] **Implement Function Composition**

  - Parse function compositions using the `.` operator, e.g., `(f . g)`
  - Define operator precedence and associativity for `.`
  - Update AST to include function composition nodes

- [x] **Handle the `.` Operator in Lexer and Parser**

  - Ensure `.` is tokenized as `Token::Dot`
  - Distinguish between `.` used for member access and function composition
  - Update parsing rules accordingly

- [x] **Update and Run Tests**

  - Add additional test cases specifically covering member access and function composition
  - Ensure all existing tests still pass

- [x] **Refactor Parser for Maintainability**

  - Clean up and organize parsing functions
  - Potentially optimize code for better readability/performance

- [ ] **Document New Features**
  - Update project docs to cover member access and function composition
  - Provide example code demonstrating each new feature
