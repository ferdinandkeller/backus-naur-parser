use super::sequence::{parse_sequence, Sequence};

#[derive(Debug, Clone)]
pub struct Alternation {
    pub sequences: Vec<Sequence>,
}

impl super::format::Format for Alternation {
    fn format(
        &self,
        output: &mut dyn std::fmt::Write,
        grammar: &super::grammar::Grammar,
    ) -> std::fmt::Result {
        let size = self.sequences.len();
        for i in 0..size {
            self.sequences[i].format(output, grammar)?;
            if i != size - 1 {
                write!(output, " | ")?;
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
    labels: &mut std::collections::HashMap<usize, String>,
    labels_reverse: &mut std::collections::HashMap<String, usize>,
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
