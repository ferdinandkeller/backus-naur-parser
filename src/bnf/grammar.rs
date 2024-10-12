use super::alternation::{parse_alternations, Alternations};
use super::element::reference::parse_reference;
use super::symbols::{parse_newlines, parse_spacings};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grammar(HashMap<String, Alternations>);

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (label, alternations) in self.0.iter() {
            write!(f, "<{}> ::= {}\n", label, alternations)?;
        }
        Ok(())
    }
}

fn parse_match_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
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

fn parse_expression(chars: &Vec<char>, index: usize) -> Result<(usize, String, Alternations), ()> {
    let (index, label) = parse_reference(chars, parse_spacings(chars, index))?;
    let index = parse_match_symbol(chars, parse_spacings(chars, index))?;
    let (index, alternations) = parse_alternations(chars, index)?;
    Ok((index, label, alternations))
}

pub fn parse_grammar(chars: &Vec<char>, mut index: usize) -> Result<(usize, Grammar), ()> {
    let mut grammar = HashMap::new();
    while let Ok((new_index, label, alternations)) = parse_expression(chars, index) {
        grammar.insert(label, alternations);
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
