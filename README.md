# gnalang

A [*blazingly fast*](https://www.rust-lang.org/) compiler for an arbitrary
language which will be revealed when the assignment for [Compiler Theory and
Practice](https://www.um.edu.mt/courses/studyunit/CPS2000) is released.

This assignment is submitted in partial fulfilment of the requirements of the
degree of [B.Sc. (Hons.) Mathematics and Computer
Science](https://www.um.edu.mt/courses/overview/ubschcgcmat-2024-5-o/) at the
[University of Malta](https://um.edu.mt/).

> [!TIP]
> This assignment scored perfect marks!

## Tasks

- [x] Table-driven lexer
- [x] Recursive descent LL(1) parser
- [x] `Visitor` trait for AST traversal
- [x] Semantic analyzer
  - [x] Scope checking
  - [x] Type checking
- [x] Assembly-like code generation
- [x] Array support

## Features

- [x] Syncronization during compilation stages, i.e. the compiler should not
crash on the first error, but should keep processing on a best-effort basis to
report as many *real* errors as possible
  - [x] Lexer syncronization
  - [ ] Parser syncronization
  - [x] Semantic analyzer syncronization
- [x] Errors are reported with line and column numbers
- [x] Basic types like `int`, `float`, `bool`, 'colour', as well as helper types like `void` and `unknown`
- [x] Recursion!
