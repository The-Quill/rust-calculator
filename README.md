## Rust calculator

Quick exercise to learn Rust by writing a calculator.

Fundamentally, a calculator is the simplest form of compiler.

A very simple tokenizer & parser, no need for a symbol table or to handle scoping.

### Usage

to setup, first install cargo, then run `cargo run` to start the calculator

### Workflow

General execution is as follows:

1. Listen to input in terminal
2. Parse the parts of the input using regular expressions:
    a. left-hand side
    b. operator
    c. right-hand side
3. Validate the input matches expectations
4. Execute the action associated to the operator
5. Log the output of the execution