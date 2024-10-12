use super::sequence::{parse_sequence, Sequence};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Alternatives(Vec<Sequence>);

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

fn parse_alternative_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('|') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

pub fn parse_alternatives(
    chars: &Vec<char>,
    mut index: usize,
) -> Result<(usize, Alternatives), ()> {
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
