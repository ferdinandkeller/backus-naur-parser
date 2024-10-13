mod bnf;

use bnf::{format::Format, grammar::parse_grammar};
use std::fs;

fn main() {
    // example
    let example = "equation_optimized";

    // load the grammar
    let raw_content =
        fs::read_to_string(format!("examples/{example}.bnf")).expect("Error reading grammar file.");
    let chars: Vec<char> = raw_content.chars().collect();
    let grammar: bnf::grammar::Grammar = parse_grammar(&chars, 0).expect("Error parsing grammar.");
    let mut out = String::with_capacity(1000);
    grammar
        .format(&mut out, &grammar)
        .expect("Error formatting grammar.");
    println!("{out}");

    // load the input
    let input =
        fs::read_to_string(format!("examples/{example}.txt")).expect("Error reading input file.");
    println!("{}\n", input);

    // try matching both
    let parsing_result = grammar.parse(&input);

    match parsing_result {
        Ok(_) => {
            println!("Parsing successful!");
        }
        Err(reason) => {
            println!("Parsing failed!");
            println!("reason : {reason:?}");
        }
    }
}
