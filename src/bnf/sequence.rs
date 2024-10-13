use super::element::{parse_element, Element};
use super::symbols::parse_spacings;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub elements: Vec<Element>,
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.elements.len();
        for i in 0..size {
            write!(f, "{}", self.elements[i])?;
            if i != size - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

pub fn parse_sequence(
    chars: &Vec<char>,
    mut index: usize,
    labels: &mut HashMap<usize, String>,
    labels_reverse: &mut HashMap<String, usize>,
) -> Result<(usize, Sequence), ()> {
    let mut elements = Vec::new();
    while let Ok((new_index, element)) =
        parse_element(chars, parse_spacings(chars, index), labels, labels_reverse)
    {
        index = new_index;
        elements.push(element);
    }
    if elements.is_empty() {
        return Err(());
    } else {
        Ok((parse_spacings(chars, index), Sequence { elements }))
    }
}
