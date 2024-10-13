pub mod empty;
pub mod literal;
pub mod range;
pub mod reference;

use empty::parse_empty_symbol;
use literal::parse_literal;
use range::parse_range;
use reference::parse_reference;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum Element {
    Empty,
    Range { start: char, end: char },
    Literal(String),
    Reference(usize),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Empty => write!(f, "ε"),
            Element::Range { start, end } => write!(f, r#""{}"..="{}""#, start, end),
            Element::Literal(s) => write!(f, r#""{s}""#),
            Element::Reference(s) => write!(f, "<{}>", s),
        }
    }
}

pub fn parse_element(
    chars: &Vec<char>,
    index: usize,
    labels: &mut HashMap<usize, String>,
    labels_reverse: &mut HashMap<String, usize>,
) -> Result<(usize, Element), ()> {
    // try to parse empty
    if let Ok(index) = parse_empty_symbol(chars, index) {
        return Ok((index, Element::Empty));
    }

    // try to parse range
    if let Ok((index, start, end)) = parse_range(chars, index) {
        return Ok((index, Element::Range { start, end }));
    }

    // try to parse literal
    if let Ok((index, string)) = parse_literal(chars, index) {
        return Ok((index, Element::Literal(string)));
    }

    // try to parse reference
    if let Ok((index, reference)) = parse_reference(chars, index, labels, labels_reverse) {
        return Ok((index, Element::Reference(reference)));
    }

    // nothing worked
    Err(())
}
