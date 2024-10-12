pub fn parse_empty_symbol(chars: &Vec<char>, index: usize) -> Result<usize, ()> {
    let Some('ε') = chars.get(index) else {
        return Err(());
    };
    Ok(index + 1)
}
