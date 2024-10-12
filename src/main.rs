mod bnf;

use bnf::grammar::parse_grammar;
use std::fs;

fn main() {
    let raw_content =
        fs::read_to_string("examples/equation.bnf").expect("Error reading grammar file");
    let chars: Vec<char> = raw_content.chars().collect();
    let (_, grammar) = parse_grammar(&chars, 0).expect("Error parsing grammar");
    println!("{}", grammar);

    // let input = fs::read_to_string("input.txt").expect("Error reading input file");
    // println!("{}", input);
}
