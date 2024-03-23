# gnalang

A [*blazingly fast*](https://www.rust-lang.org/) compiler for an arbitrary
language which will be revealed when the assignment for [Compiler Theory and
Practice](https://www.um.edu.mt/courses/studyunit/CPS2000) is released.

This assignment is submitted in partial fulfilment of the requirements of the
degree of [B.Sc. (Hons.) Mathematics and Computer
Science](https://www.um.edu.mt/courses/overview/ubschcgcmat-2024-5-o/) at the
[University of Malta](https://um.edu.mt/).

## Features

- [x] Table-driven lexer
- [x] Recursive descent LL(1) parser
- [x] Abstract `Visitor` trait for AST traversal
- [x] Semantic analyzer
  - [x] Scope checking
  - [x] Type checking
- [ ] Assembly-like code generation
