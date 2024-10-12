use std::{collections::HashMap, fmt::Display, fs};

fn main() {
    // let raw_content = fs::read_to_string("bnf.txt").expect("Error reading grammar file");
    // let chars: Vec<char> = raw_content.chars().collect();
    // let (_, grammar) = parse_grammar(&chars, 0).expect("Error parsing grammar");
    // println!("{}", grammar);

    // let input = fs::read_to_string("input.txt").expect("Error reading input file");
    // println!("{}", input);
}

#[derive(Debug, Clone)]
struct Grammar(HashMap<String, Alternatives>);

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (label, alternatives) in self.0.iter() {
            write!(f, "<{}> ::= {}\n", label, alternatives)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Expression {
    label: String,
    alternatives: Alternatives,
}

#[derive(Debug, Clone)]
struct Alternatives(Vec<Sequence>);

impl Display for Alternatives {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len();
        for i in 0..size {
            write!(f, "{}", self.0[i])?;
            if i != size - 1 {
                write!(f, " | ")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Sequence(Vec<Element>);

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len();
        for i in 0..size {
            write!(f, "{}", self.0[i])?;
            if i != size - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Element {
    Literal(String),
    Range { start: char, end: char },
    Reference(String),
    Nothing,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Literal(s) => write!(f, r#""{s}""#),
            Element::Range { start, end } => write!(f, r#""{}"..="{}""#, start, end),
            Element::Reference(s) => write!(f, "<{}>", s),
            Element::Nothing => write!(f, "ε"),
        }
    }
}

fn parse_spacings(chars: &Vec<char>, mut index: usize) -> usize {
    while let Some(' ') = chars.get(index) {
        index += 1;
    }
    index
}

fn parse_single_newline(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    match chars.get(index) {
        Some('\n') => Ok(index + 1),
        Some('\r') => match chars.get(index + 1) {
            Some('\n') => Ok(index + 2),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn parse_newlines(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let mut index = parse_single_newline(chars, index)?;
    while let Ok(new_index) = parse_single_newline(chars, index) {
        index += new_index;
    }
    Ok(index)
}

fn parse_reference(chars: &Vec<char>, mut index: usize) -> Result<(usize, String), ()> {
    let Some('<') = chars.get(index) else {
        return Err(());
    };

    let mut reference = String::new();
    index += 1;

    while let Some(c) = chars.get(index) {
        if c == &'>' {
            break;
        }
        reference.push(*c);
        index += 1;
    }

    if let Some('>') = chars.get(index) {
        Ok((index + 1, reference))
    } else {
        Err(())
    }
}

fn parse_match(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some(':') = chars.get(index) else {
        return Err(());
    };
    let Some(':') = chars.get(index + 1) else {
        return Err(());
    };
    let Some('=') = chars.get(index + 2) else {
        return Err(());
    };
    Ok(index + 3)
}

fn parse_string(chars: &Vec<char>, index: usize) -> Result<(usize, String), ()> {
    // parse the start of the string
    let (mut index, escape_length) = parse_string_start(chars, index)?;

    // parse the content of the string
    let mut content = String::new();
    loop {
        match chars.get(index) {
            Some(c) => {
                if c == &'"' {
                    if let Ok(index) = parse_string_end(chars, index, escape_length) {
                        return Ok((index, content));
                    }
                }
                content.push(*c);
                index += 1;
            }
            None => return Err(()),
        }
    }
}

fn parse_string_start(chars: &Vec<char>, index: usize) -> Result<(usize, usize), ()> {
    // count how many # there are before the string
    let mut escape_length = 0;
    while let Some('#') = chars.get(index + escape_length) {
        escape_length += 1;
    }

    // check that there is a " after the #
    let Some('"') = chars.get(index + escape_length) else {
        return Err(());
    };

    // return the new index & the escape length
    Ok((index + escape_length + 1, escape_length))
}

fn parse_string_end(chars: &Vec<char>, index: usize, escape_length: usize) -> Result<usize, ()> {
    // check that there is a "
    let Some('"') = chars.get(index) else {
        return Err(());
    };

    // check that there are enough # after the "
    for i in 0..escape_length {
        let Some('#') = chars.get(index + 1 + i) else {
            return Err(());
        };
    }

    // return the new index
    Ok(index + 1 + escape_length)
}

fn parse_range(chars: &Vec<char>, index: usize) -> Result<(usize, char, char), ()> {
    let (index, first_string) = parse_string(chars, index)?;
    if first_string.len() != 1 {
        return Err(());
    }

    let index = parse_range_symbol(chars, index)?;

    let (index, second_string) = parse_string(chars, index)?;
    if second_string.len() != 1 {
        return Err(());
    }

    Ok((
        index,
        first_string
            .chars()
            .nth(0)
            .expect("could not get first char"),
        second_string
            .chars()
            .nth(0)
            .expect("could not get second char"),
    ))
}

fn parse_range_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('.') = chars.get(index) else {
        return Err(());
    };
    let Some('.') = chars.get(index + 1) else {
        return Err(());
    };
    let Some('=') = chars.get(index + 2) else {
        return Err(());
    };
    Ok(index + 3)
}

fn parse_nothing_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('ε') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

fn parse_element(chars: &Vec<char>, index: usize) -> Result<(usize, Element), ()> {
    // try to parse nothing
    if let Ok(index) = parse_nothing_symbol(chars, index) {
        return Ok((index, Element::Nothing));
    }

    // try to parse range
    if let Ok((index, start, end)) = parse_range(chars, index) {
        return Ok((index, Element::Range { start, end }));
    }

    // try to parse string
    if let Ok((index, string)) = parse_string(chars, index) {
        return Ok((index, Element::Literal(string)));
    }

    // try to parse reference
    if let Ok((index, reference)) = parse_reference(chars, index) {
        return Ok((index, Element::Reference(reference)));
    }

    // nothing worked
    Err(())
}

fn parse_sequence(chars: &Vec<char>, mut index: usize) -> Result<(usize, Sequence), ()> {
    let mut elements = Vec::new();
    while let Ok((new_index, element)) = parse_element(chars, parse_spacings(chars, index)) {
        index = new_index;
        elements.push(element);
    }
    if elements.is_empty() {
        return Err(());
    } else {
        Ok((parse_spacings(chars, index), Sequence(elements)))
    }
}

fn parse_alternative_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('|') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

fn parse_alternatives(chars: &Vec<char>, mut index: usize) -> Result<(usize, Alternatives), ()> {
    let mut sequences = Vec::new();
    while let Ok((new_index, sequence)) = parse_sequence(chars, index) {
        sequences.push(sequence);
        index = new_index;
        match parse_alternative_symbol(chars, new_index) {
            Ok(new_index) => index = new_index,
            Err(_) => break,
        }
    }
    if sequences.is_empty() {
        return Err(());
    } else {
        Ok((index, Alternatives(sequences)))
    }
}

fn parse_expression(chars: &Vec<char>, index: usize) -> Result<(usize, Expression), ()> {
    let (index, label) = parse_reference(chars, parse_spacings(chars, index))?;
    let index = parse_match(chars, parse_spacings(chars, index))?;
    let (index, alternatives) = parse_alternatives(chars, index)?;
    Ok((
        index,
        Expression {
            label,
            alternatives,
        },
    ))
}

fn parse_grammar(chars: &Vec<char>, mut index: usize) -> Result<(usize, Grammar), ()> {
    let mut grammar = HashMap::new();
    while let Ok((new_index, expression)) = parse_expression(chars, index) {
        grammar.insert(expression.label, expression.alternatives);
        index = new_index;
        match parse_newlines(chars, index) {
            Ok(new_index) => index = new_index,
            Err(_) => break,
        }
    }
    if chars.len() != index {
        return Err(());
    }
    Ok((index, Grammar(grammar)))
}
