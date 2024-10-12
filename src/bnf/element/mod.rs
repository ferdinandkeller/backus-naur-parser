pub mod empty;
pub mod range;
pub mod reference;
pub mod string;

use empty::parse_empty_symbol;
use range::parse_range;
use reference::parse_reference;
use std::fmt::Display;
use string::parse_string;

#[derive(Debug, Clone)]
pub enum Element {
    Literal(String),
    Range { start: char, end: char },
    Reference(String),
    Empty,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Literal(s) => write!(f, r#""{s}""#),
            Element::Range { start, end } => write!(f, r#""{}"..="{}""#, start, end),
            Element::Reference(s) => write!(f, "<{}>", s),
            Element::Empty => write!(f, "ε"),
        }
    }
}

pub fn parse_element(chars: &Vec<char>, index: usize) -> Result<(usize, Element), ()> {
    // try to parse empty
    if let Ok(index) = parse_empty_symbol(chars, index) {
        return Ok((index, Element::Empty));
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
