# Backus Naur Form Grammar Parser

The goal of this project is to write a parser in Rust capable of reading a valid Backup Naur Form (BNF) grammar, and applying it to a given input file. BNF in itself is quite limited, but is a good starting point for more advanced parsing algorithms.

Here is a sample valid BNF grammar:

```txt
<expr> ::= <term> | <term> <opt-space> "+" <opt-space> <expr> | <term> <opt-space> "-" <opt-space> <expr>
<term> ::= <factor> | <factor> <opt-space> "*" <opt-space> <term> | <factor> <opt-space> "/" <opt-space> <term>
<factor> ::= "(" <opt-space> <expr> <opt-space> ")" | <number>
<number> ::= <digit> <number> | <digit>
<digit> ::= "0"..="9"
<opt-space> ::= " " <opt-space> | ε
```

Along with an input file:
```txt
(21 + 35) * 123 + 4*4
```

And the generated output:
```txt

```

> Note : The entrypoint will be the first non-terminal symbol defined in the grammar.

BNF itself is quite simple and doesn't require advanced backtracking algorithms to be parsed. A simpel REGEX would probably do the job (though I am not using them here because that would defeat the purpose). But that isn't necessarily the case for the grammar defined in the BNF file itself. Parsing unknown grammars might require advanced backtracking algorithms. I tried to keep the logic as simple and documented as possible.

Optimization could be done in many ways (notably not using constant heap allocation).