use super::alternation::{parse_alternations, Alternation};
use super::element::reference::parse_reference;
use super::element::Element;
use super::symbols::{parse_newlines, parse_spacings};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub references: HashMap<usize, String>,
    pub labels: HashSet<usize>,
    pub maps: HashMap<usize, Alternation>,
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for label_index in 1..self.references.len() {
            let label = self
                .references
                .get(&label_index)
                .expect("Label should exist.");
            let alternation = self
                .maps
                .get(&label_index)
                .expect("Alternation should exist.");
            write!(f, "{label} ::= {alternation}\n",)?;
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

fn parse_expression(
    chars: &Vec<char>,
    index: usize,
    labels: &mut HashMap<usize, String>,
    labels_reverse: &mut HashMap<String, usize>,
) -> Result<(usize, usize, Alternation), ()> {
    let (index, label_index) =
        parse_reference(chars, parse_spacings(chars, index), labels, labels_reverse)?;
    let index = parse_match_symbol(chars, parse_spacings(chars, index))?;
    let (index, alternations) = parse_alternations(chars, index, labels, labels_reverse)?;
    Ok((index, label_index, alternations))
}

pub fn parse_grammar(chars: &Vec<char>, mut index: usize) -> Result<Grammar, ()> {
    let mut references = HashMap::new();
    let mut references_reversed = HashMap::new();
    let mut labels = HashSet::new();
    let mut maps = HashMap::new();

    // parse all the expressions
    while let Ok((new_index, label_index, alternations)) =
        parse_expression(chars, index, &mut references, &mut references_reversed)
    {
        // check if the label was already defined
        if labels.contains(&label_index) {
            return Err(());
        } else {
            labels.insert(label_index);
        }

        maps.insert(label_index, alternations);
        index = new_index;
        match parse_newlines(chars, index) {
            Ok(new_index) => index = new_index,
            Err(_) => break,
        }
    }

    // if we didn't parse all the input characters, then the grammar is invalid
    if chars.len() != index {
        return Err(());
    }

    // if there are no labels, then the grammar is invalid
    if labels.is_empty() {
        return Err(());
    }

    // if the number of labels doesn't match the number of references, then the grammar is invalid
    if references.len() != labels.len() {
        return Err(());
    }

    Ok(Grammar {
        references,
        labels,
        maps,
    })
}

#[derive(Debug, Clone, Copy)]
pub struct ChoiceState {
    depth: usize,
    source_label_index: usize,
    source_alternation_index: usize,
    source_sequence_index: usize,
    destination_label_index: usize,
    destination_alternation_index: usize,
    input_index: usize,
}

impl Display for ChoiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{},{}]→{}[{}]",
            self.source_label_index,
            self.source_alternation_index,
            self.source_sequence_index,
            self.destination_label_index,
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
            source_label_index: 0, // this label doesn't exist, it's just a placeholder for the entrypoint
            source_alternation_index: 0,
            source_sequence_index: 0,
            destination_label_index: 1, // the first label defined is the entrypoint
            destination_alternation_index: 0,
            input_index: 0,
        }];

        let mut current_depth: usize = 1;
        let mut current_label_index: usize = 1;
        let mut current_alternation_index: usize = 0;
        let mut current_sequence_index: usize = 0;
        let mut current_input_index: usize = 0;

        'main_loop: loop {
            // check for stack overflow
            if choice_stack.len() > 1_000_000 {
                return Err(Error::StackOverflow);
            }

            // retrieve the current alternation
            let current_alternation = self
                .maps
                .get(&current_label_index)
                .expect("Label should exist.");

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
                current_label_index = choice.destination_label_index;
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
                    current_label_index = last_choice.destination_label_index;
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
                current_label_index = previous_choice.source_label_index;
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
                Element::Reference(label_index) => {
                    // instead of continuing the loop, we go one step deeper and reset
                    choice_stack.push(ChoiceState {
                        depth: current_depth,
                        source_label_index: current_label_index,
                        source_alternation_index: current_alternation_index,
                        source_sequence_index: current_sequence_index,
                        destination_label_index: *label_index,
                        destination_alternation_index: 0,
                        input_index: current_input_index,
                    });

                    current_depth += 1;
                    current_label_index = label_index.clone();
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
                    current_label_index = last_choice.destination_label_index.clone();
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
