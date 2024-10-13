mod bnf;

use bnf::grammar::parse_grammar;
use std::fs;

fn main() {
    // example
    let example = "equation";

    // load the grammar
    let raw_content =
        fs::read_to_string(format!("examples/{example}.bnf")).expect("Error reading grammar file.");
    let chars: Vec<char> = raw_content.chars().collect();
    let grammar = parse_grammar(&chars, 0).expect("Error parsing grammar.");
    println!("{}", grammar);

    // load the input
    let input =
        fs::read_to_string(format!("examples/{example}.txt")).expect("Error reading input file.");
    println!("{}\n", input);

    // try matching both
    let parsing_result = grammar.parse(&input);

    match parsing_result {
        Ok(choice_states) => {
            println!("Parsing successful!");
            for choice_state in choice_states.into_iter() {
                println!("{}", choice_state);
            }
        }
        Err(reason) => {
            println!("Parsing failed!");
            println!("reason : {reason:?}");
        }
    }
}
