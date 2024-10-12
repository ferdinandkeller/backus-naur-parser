# Backus Naur Parser

The goal of this project is to write a parser in Rust capable of parsing any valid Backup Naur Form (BNF) grammar, and apply it to a given input file.

Here is a sample valid BNF grammar, along with an input file, and the expected output:

```txt
<digit> ::= "0"..="9"
<number> ::= <digit> <number> | <digit>
<space> ::= " " <space> | " " | ε
<factor> ::= "(" <space> <expr> <space> ")" | <number>
<term> ::= <factor> | <factor> <space> "*" <space> <term> | <factor> <space> "/" <space> <term>
<expr> ::= <term> | <term> <space> "+" <space> <expr> | <term> <space> "-" <space> <expr>
```

```txt
(21 + 35) * 123 + 4*4
```