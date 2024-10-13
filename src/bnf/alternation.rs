use super::sequence::{parse_sequence, Sequence};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Alternation {
    pub sequences: Vec<Sequence>,
}

impl Display for Alternation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.sequences.len();
        for i in 0..size {
            write!(f, "{}", self.sequences[i])?;
            if i != size - 1 {
                write!(f, " | ")?;
            }
        }
        Ok(())
    }
}

fn parse_alternation_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('|') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

pub fn parse_alternations(
    chars: &Vec<char>,
    mut index: usize,
    labels: &mut HashMap<usize, String>,
    labels_reverse: &mut HashMap<String, usize>,
) -> Result<(usize, Alternation), ()> {
    let mut sequences = Vec::new();
    while let Ok((new_index, sequence)) = parse_sequence(chars, index, labels, labels_reverse) {
        sequences.push(sequence);
        index = new_index;
        match parse_alternation_symbol(chars, new_index) {
            Ok(new_index) => index = new_index,
            Err(_) => break,
        }
    }
    if sequences.is_empty() {
        return Err(());
    } else {
        Ok((index, Alternation { sequences }))
    }
}
