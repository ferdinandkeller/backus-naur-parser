use std::collections::HashMap;

fn parse_openbra_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('<') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

fn parse_closebra_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('>') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}

pub fn parse_reference(
    chars: &Vec<char>,
    index: usize,
    labels: &mut HashMap<usize, String>,
    labels_reverse: &mut HashMap<String, usize>,
) -> Result<(usize, usize), ()> {
    let mut index = parse_openbra_symbol(chars, index)?;

    let mut reference = String::new();
    loop {
        if let Ok(index) = parse_closebra_symbol(chars, index) {
            // find the index of the reference
            let reference_index = match labels_reverse.get(&reference) {
                Some(&reference_index) => reference_index,
                None => {
                    // the "+1" is necessary because index 0 is reserved for the entrypoint
                    // one could theoretically insert some random String, but then the value would be somewhat arbitrary
                    let reference_index = labels.len() + 1;
                    labels.insert(reference_index, reference.clone());
                    labels_reverse.insert(reference.clone(), reference_index);
                    reference_index
                }
            };
            return Ok((index, reference_index));
        }

        match chars.get(index) {
            Some(c) => {
                reference.push(*c);
                index += 1;
            }
            None => return Err(()),
        }
    }
}
