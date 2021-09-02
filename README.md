# CLASP

Basically Lisp but compiles into a type of bytecode like JRE and has interpreter.

- No null
- Every statement has a return type
- No void return type (Maybe)

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
(printf "This is cool, %s\n" "name")
(def name
  (return "bob"))
```
