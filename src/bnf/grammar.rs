use super::alternation::{self, parse_alternations, Alternation};
use super::element::reference::parse_reference;
use super::element::Element;
use super::sequence::{self, Sequence};
use super::symbols::{parse_newlines, parse_spacings};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grammar {
    labels: Vec<String>,
    maps: HashMap<String, Alternation>,
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for label in self.labels.iter() {
            write!(f, "{} ::= {}\n", label, self.maps.get(label).unwrap())?;
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

fn parse_expression(chars: &Vec<char>, index: usize) -> Result<(usize, String, Alternation), ()> {
    let (index, label) = parse_reference(chars, parse_spacings(chars, index))?;
    let index = parse_match_symbol(chars, parse_spacings(chars, index))?;
    let (index, alternations) = parse_alternations(chars, index)?;
    Ok((index, label, alternations))
}

pub fn parse_grammar(chars: &Vec<char>, mut index: usize) -> Result<Grammar, ()> {
    let mut labels = vec![];
    let mut maps = HashMap::new();
    while let Ok((new_index, label, alternations)) = parse_expression(chars, index) {
        labels.push(label.clone());
        maps.insert(label, alternations);
        index = new_index;
        match parse_newlines(chars, index) {
            Ok(new_index) => index = new_index,
            Err(_) => break,
        }
    }
    if chars.len() != index {
        return Err(());
    }
    if maps.is_empty() {
        return Err(());
    }
    Ok(Grammar { labels, maps })
}

#[derive(Debug, Clone)]
pub struct ChoiceState {
    depth: usize,
    source_label: String,
    source_alternation_index: usize,
    source_sequence_index: usize,
    destination_label: String,
    destination_alternation_index: usize,
    input_index: usize,
}

impl Display for ChoiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{},{}]→{}[{}]",
            self.source_label,
            self.source_alternation_index,
            self.source_sequence_index,
            self.destination_label,
            self.destination_alternation_index
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    StackOverflow,
    NoMatch,
}

impl Grammar {
    pub fn parse(&self, input: &str) -> Result<Vec<ChoiceState>, Error> {
        let chars: Vec<char> = input.chars().collect();

        let mut choice_stack: Vec<ChoiceState> = vec![ChoiceState {
            depth: 0,
            source_label: self.labels[0].clone(),
            source_alternation_index: 0,
            source_sequence_index: 0,
            destination_label: self.labels[0].clone(),
            destination_alternation_index: 0,
            input_index: 0,
        }];

        let mut current_depth: usize = 1;
        let mut current_label: String = self.labels[0].clone();
        let mut current_alternation_index: usize = 0;
        let mut current_sequence_index: usize = 0;
        let mut current_input_index: usize = 0;

        'main_loop: loop {
            // check for stack overflow
            if choice_stack.len() > 1_000_000 {
                return Err(Error::StackOverflow);
            }

            // retrieve the current alternation
            let current_alternation = self.maps.get(&current_label).expect("Label should exist.");

            // check that the current alternation index isn't out of bounds
            if current_alternation.sequences.len() <= current_alternation_index {
                // dump the previous choice
                choice_stack
                    .pop()
                    .expect("Choice stack should not be empty.");

                // update the choice before that
                let Some(choice) = choice_stack.last_mut() else {
                    // if we are at the entry of the grammar, but we failled all the alternations,
                    // then there are no more options, and the parsing failed
                    return Err(Error::NoMatch);
                };

                // go to the next alternation
                choice.destination_alternation_index += 1;

                // move cursor to there
                current_depth = choice.depth + 1;
                current_label = choice.destination_label.clone();
                current_alternation_index = choice.destination_alternation_index;
                current_sequence_index = 0;
                current_input_index = choice.input_index;

                // continue the loop
                continue 'main_loop;
            }

            // pull the sequence
            let current_sequence = &current_alternation.sequences[current_alternation_index];

            // check that the current sequence index isn't out of bounds
            // if we are out of bounds it means we completed the sequence
            if current_sequence.elements.len() <= current_sequence_index {
                // if we are at level 1 ...
                if current_depth == 1 {
                    // ... either we parsed all the input text, and we are done
                    if current_input_index == chars.len() {
                        return Ok(choice_stack);
                    }

                    // ... or we didn't, in which case we need to backtrack

                    // ref to the last choice
                    let last_choice = choice_stack
                        .last_mut()
                        .expect("Choice stack should not be empty.");

                    // update it to go to the next alternation (because the current one failed)
                    last_choice.destination_alternation_index += 1;

                    // move cursor to there
                    current_depth = last_choice.depth + 1;
                    current_label = last_choice.destination_label.clone();
                    current_alternation_index = last_choice.destination_alternation_index;
                    current_sequence_index = 0;
                    current_input_index = last_choice.input_index;

                    continue 'main_loop;
                }

                // if we aren't at level 1, we need to go one level up
                let target_depth = current_depth - 1;
                let mut stack_size: usize = choice_stack.len() - 1;
                let x;
                loop {
                    if choice_stack[stack_size].depth == target_depth {
                        x = stack_size;
                        break;
                    }
                    if stack_size == 0 {
                        panic!("Choice stack should not be empty.");
                    } else {
                        stack_size -= 1;
                    }
                }

                // reference to the choice one level up
                let previous_choice = &choice_stack[x];

                current_depth = previous_choice.depth;
                current_label = previous_choice.source_label.clone();
                current_alternation_index = previous_choice.source_alternation_index;
                current_sequence_index = previous_choice.source_sequence_index + 1;
                // we don't reset the input index because we want to continue where we left off

                continue 'main_loop;
            }

            // pull the element
            let current_element = &current_sequence.elements[current_sequence_index];

            // we match the element
            let match_result: Result<usize, ()> = match current_element {
                // match the empty element (always true)
                Element::Empty => Ok(current_input_index),
                // match the range element
                Element::Range { start, end } => {
                    match_range(start, end, &chars, current_input_index)
                }
                // match the literal element
                Element::Literal(literal) => match_literal(literal, &chars, current_input_index),
                // the reference element is a bit special
                Element::Reference(label) => {
                    // instead of continuing the loop, we go one step deeper and reset
                    choice_stack.push(ChoiceState {
                        depth: current_depth,
                        source_label: current_label.clone(),
                        source_alternation_index: current_alternation_index,
                        source_sequence_index: current_sequence_index,
                        destination_label: label.clone(),
                        destination_alternation_index: 0,
                        input_index: current_input_index,
                    });

                    current_depth += 1;
                    current_label = label.clone();
                    current_alternation_index = 0;
                    current_sequence_index = 0;

                    continue 'main_loop;
                }
            };

            match match_result {
                // if the match succeeded, then we update the input cursor and bump the sequence index
                Ok(new_input_index) => {
                    current_input_index = new_input_index;
                    current_sequence_index += 1;
                }
                // if the match failed
                Err(()) => {
                    // ref to the last choice
                    let last_choice = choice_stack
                        .last_mut()
                        .expect("Choice stack should not be empty.");

                    // update it to go to the next alternation (because the current one failed)
                    last_choice.destination_alternation_index += 1;

                    // move cursor to there
                    current_depth = last_choice.depth + 1;
                    current_label = last_choice.destination_label.clone();
                    current_alternation_index = last_choice.destination_alternation_index;
                    current_sequence_index = 0;
                    current_input_index = last_choice.input_index;
                }
            }
        }
    }
}

fn match_range(start: &char, end: &char, chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    if let Some(c) = chars.get(index) {
        if start <= c && c <= end {
            Ok(index + 1)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn match_literal(literal: &str, chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let mut i = 0;
    for c in literal.chars() {
        if let Some(&input_char) = chars.get(index + i) {
            if c != input_char {
                return Err(());
            }
        } else {
            return Err(());
        }
        i += 1;
    }
    Ok(index + i)
}
