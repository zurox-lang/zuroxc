# Zurox Compiler

## Why Zurox ?

You can judge it yourself.

## Why was the codebase migrated from C++ to Rust ?

There are three primary reasons:
- Rust supports UTF-8 encoded strings by default. This saves a lot of trouble encountered with ICU integration.
- Safety while dealing with references. Using smart pointers complicated the code-base a lot, and also made it harder to write new-code by having to worry about memory related issues.
- The most important reason, I really liked Rust's `struct` and `enum` variants, so I decided it was worth enough to switch to it.

But why did it take so long to realize ?
- Skill issue
- I was worried about LLVM integration, because at the time of trying `llvm-sys` crate, I had a lot of difficulties using it. I decided to use the LLVM C/C++ headers instead.

## Languages/Tools
- `Rust` (`C++` originally)
- Front-end - Custom
- Back-end - `LLVM` (Hopefully) [Details](https://llvm.org). C for alternative back-end.
## What will happen if I can't integrate with LLVM ?

Well, I will just write a C-backend. Or if I am too lazy, I will ditch the compiler and just write an interpreter.

## Should you use this for serious projects ?

If you are wondering/pondering about this, you definitely should not.

## Road Map

This tracks the progress of the compiler:

1. Grammar
- [x] Define the grammar in E-BNF.
- [x] Decide on the functionality and choice of keywords.
- [x] Remove ambiguity and left-recursion.
- [ ] Stabilize grammar of the language.

2. Lexical Analysis
- [x] Create the Lexer for Lexical analysis.
- [x] Test the Lexer.
- [ ] Finalize the Lexer.

3. AST
- [x] Create the Abstract Syntax Tree.
- [ ] Stabilize and finalize the AST.

4. Parsing
- [ ] Create the Parser for parsing.
- [ ] Test the Parser.
- [ ] Finalize the parser.

5. Semantical Analysis
- [ ] Create the symbol table.
- [ ] Test the symbol table.
- [ ] Finalize the symbol table.
- [ ] Create the static analyzer.
- [ ] Test the static analyzer.
- [ ] Finalize the static analyzer.
- [ ] Create the borrow checker.
- [ ] Test the borrow checker.
- [ ] Finalize the borrow checker.

6. IR
- [ ] Create stable interface for creation and managing the IR.
- [ ] Make sure that the translation is "one-one" and does not change or lose **ANY** details during the process.
- [ ] Test the interface.

7. Optimization

> To be decided later.

8. Machine Code/Bit-Code

>To be decided later.
