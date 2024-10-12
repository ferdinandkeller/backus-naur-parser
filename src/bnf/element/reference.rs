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

pub fn parse_reference(chars: &Vec<char>, index: usize) -> Result<(usize, String), ()> {
    let mut index = parse_openbra_symbol(chars, index)?;

    let mut reference = String::new();
    loop {
        if let Ok(index) = parse_closebra_symbol(chars, index) {
            return Ok((index, reference));
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
