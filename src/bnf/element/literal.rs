fn parse_literal_start(chars: &Vec<char>, index: usize) -> Result<(usize, usize), ()> {
    // count how many # there are before the string
    let mut escape_length = 0;
    while let Some('#') = chars.get(index + escape_length) {
        escape_length += 1;
    }

    // check that there is a " after the #
    let Some('"') = chars.get(index + escape_length) else {
        return Err(());
    };

    // return the new index & the escape length
    Ok((index + escape_length + 1, escape_length))
}

fn parse_literal_end(chars: &Vec<char>, index: usize, escape_length: usize) -> Result<usize, ()> {
    // check that there is a "
    let Some('"') = chars.get(index) else {
        return Err(());
    };

    // check that there are enough # after the "
    for i in 0..escape_length {
        let Some('#') = chars.get(index + 1 + i) else {
            return Err(());
        };
    }

    // return the new index
    Ok(index + 1 + escape_length)
}

pub fn parse_literal(chars: &Vec<char>, index: usize) -> Result<(usize, String), ()> {
    // parse the start of the string
    let (mut index, escape_length) = parse_literal_start(chars, index)?;

    // parse the content of the string
    let mut content = String::new();
    loop {
        if let Ok(index) = parse_literal_end(chars, index, escape_length) {
            if content.is_empty() {
                return Err(());
            }
            return Ok((index, content));
        }

        match chars.get(index) {
            Some(c) => {
                content.push(*c);
                index += 1;
            }
            None => return Err(()),
        }
    }
}
