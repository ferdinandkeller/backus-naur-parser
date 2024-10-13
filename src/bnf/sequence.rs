use super::element::{parse_element, Element};
use super::symbols::parse_spacings;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub elements: Vec<Element>,
}

impl super::format::Format for Sequence {
    fn format(
        &self,
        output: &mut dyn std::fmt::Write,
        grammar: &super::grammar::Grammar,
    ) -> std::fmt::Result {
        let size = self.elements.len();
        for i in 0..size {
            self.elements[i].format(output, grammar)?;
            if i != size - 1 {
                write!(output, " ")?;
            }
        }
        Ok(())
    }
}

pub fn parse_sequence(
    chars: &Vec<char>,
    mut index: usize,
    labels: &mut std::collections::HashMap<usize, String>,
    labels_reverse: &mut std::collections::HashMap<String, usize>,
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
