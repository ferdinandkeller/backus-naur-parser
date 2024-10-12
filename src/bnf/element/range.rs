use super::string::parse_string;

fn parse_range_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('.') = chars.get(index) else {
        return Err(());
    };
    let Some('.') = chars.get(index + 1) else {
        return Err(());
    };
    let Some('=') = chars.get(index + 2) else {
        return Err(());
    };
    Ok(index + 3)
}

pub fn parse_range(chars: &Vec<char>, index: usize) -> Result<(usize, char, char), ()> {
    let (index, first_string) = parse_string(chars, index)?;
    if first_string.len() != 1 {
        return Err(());
    }

    let index = parse_range_symbol(chars, index)?;

    let (index, second_string) = parse_string(chars, index)?;
    if second_string.len() != 1 {
        return Err(());
    }

    Ok((
        index,
        first_string
            .chars()
            .nth(0)
            .expect("could not get first char"),
        second_string
            .chars()
            .nth(0)
            .expect("could not get second char"),
    ))
}
