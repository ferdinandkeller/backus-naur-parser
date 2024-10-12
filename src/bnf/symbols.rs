pub fn parse_spacings(chars: &Vec<char>, mut index: usize) -> usize {
    while let Some(' ') = chars.get(index) {
        index += 1;
    }
    index
}

pub fn parse_single_newline(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    match chars.get(index) {
        Some('\n') => Ok(index + 1),
        Some('\r') => match chars.get(index + 1) {
            Some('\n') => Ok(index + 2),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

pub fn parse_newlines(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let mut index = parse_single_newline(chars, index)?;
    while let Ok(new_index) = parse_single_newline(chars, index) {
        index += new_index;
    }
    Ok(index)
}
