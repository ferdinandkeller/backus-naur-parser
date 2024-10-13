# Backus Naur Form Grammar Parser

The goal of this project is to write a parser in Rust capable of reading a valid Backup Naur Form (BNF) grammar, and applying it to a given input file. BNF in itself is quite limited, but is a good starting point for more advanced parsing algorithms.


## Example
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

## Grammar
The grammar is slightly non-standard, as I added :
- ranges (like "0"..="9"), based on unicode code points, making it easier to define a set of characters
- ε symbol for empty strings (it always matches, great for making stuff optional)
- advanced strings, like `#"my awesome string containing "quotes" inside"#`. The quotes will be properly escaped, and many # can be used if needed `##"Here is "another" string with #"quotes"# inside"##`. This is inspired by the Rust syntax for raw strings.

> Note : The entrypoint will be the first non-terminal symbol defined in the grammar.

## Details
BNF itself is quite simple and doesn't require advanced backtracking algorithms to be parsed. A REGEX would probably do the job (though I am not using them here because that would defeat the purpose). But that isn't necessarily the case for the grammar defined in the BNF file itself. Parsing unknown grammars might require advanced backtracking algorithms. I tried to keep the logic as simple and documented as possible.

Because the parser tries to parse the grammar in the order of definition, ordering differently the alternations & optimizing the grammar can have a tremendous impact on the performance. Check `equation.bnf` vs `equation_optimized.bnf` for an example. In the real world, you would also use a two-steps parsing, first tokenizing the input, and then parsing the tokens.