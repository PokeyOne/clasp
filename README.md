# CLASP

The main idea behind this language is it is a bit of a combination of C, Java,
and Lisp. The syntax is most like Lisp, the way the language is compiled
and run is most like Java, and the behaviour and structure is most like like C.

The project is built using Rust and Rust's package manager, Cargo, and is
divided into three components.

- **Runtime Environment**: The runner for the compiled clasp.
- **Clasm Compiler**: The program that compiles the Clasm code into a compiled `.cclasp` file that the interpreter can run.
- **Clasp Compiler**: The program that compiles the Clasp code to Clasm.

The base of the project is a cargo workspace, and each of the three components
is a cargo project. All the standard cargo commands work in terms of building
and running the project.

## Documentation

Currently documentation is light. Below is a list of documentation resources.

1. [CLASM.md](CLASM.md): This file contains documentation on programming for the clasp virtual machine in assembly. **out of date**
2. [hello_world.clasm](clasm_compiler/test_files/hello_world.clasm): This is an example clasm file that will compile with the latest compiler.

more to come

## EBNF

This is a description of the language

```ebnf
file ::= statement*
statement ::= '(' method_id parameters ')'
method_id ::= (letter | '_')+
parameters ::= ...
```

## Code Samples

```text
(include standard_io)

(typedef char* String)

(def name
  (returns char*)
  (return "bob"))

(def name_with_args
  (accept String name)
  (returns char*)
  (return (sprintf "%s" name)))

(def main
  (returns int)
  (printf "This is cool, %s\n" (name))
  (printf "but this is even cooler, %s\n" (name_with_args "boris"))
  (return 0))
```
