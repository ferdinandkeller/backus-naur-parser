pub fn parse_reference(chars: &Vec<char>, mut index: usize) -> Result<(usize, String), ()> {
    let Some('<') = chars.get(index) else {
        return Err(());
    };

    let mut reference = String::new();
    index += 1;

    while let Some(c) = chars.get(index) {
        if c == &'>' {
            break;
        }
        reference.push(*c);
        index += 1;
    }

    if let Some('>') = chars.get(index) {
        Ok((index + 1, reference))
    } else {
        Err(())
    }
}
