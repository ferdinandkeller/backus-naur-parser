fn parse_string_start(chars: &Vec<char>, index: usize) -> Result<(usize, usize), ()> {
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

fn parse_string_end(chars: &Vec<char>, index: usize, escape_length: usize) -> Result<usize, ()> {
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

pub fn parse_string(chars: &Vec<char>, index: usize) -> Result<(usize, String), ()> {
    // parse the start of the string
    let (mut index, escape_length) = parse_string_start(chars, index)?;

    // parse the content of the string
    let mut content = String::new();
    loop {
        match chars.get(index) {
            Some(c) => {
                if c == &'"' {
                    if let Ok(index) = parse_string_end(chars, index, escape_length) {
                        return Ok((index, content));
                    }
                }
                content.push(*c);
                index += 1;
            }
            None => return Err(()),
        }
    }
}
