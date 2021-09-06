# CLASP

Basically Lisp but compiles into a type of bytecode like JRE and has interpreter.

- No null
- Every statement has a return type
- No void return type (Maybe)

## Documentation

Currently documentation is light. Below is a list of documentation resources.

1. [CLASM.md](CLASM.md): This file contains documentation on programming for the clasp virtual machine in assembly.

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
