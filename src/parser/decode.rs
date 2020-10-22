use super::error::AnonymousRuntimeError;


fn decode_letter(letter: &char) -> Result<u8, AnonymousRuntimeError> {
    match letter {
        '0'..='9' => Ok((*letter as u8) - ('0' as u8)),
        'A'..='F' => Ok((*letter as u8) - ('A' as u8) + 10),
        'a'..='f' => Ok((*letter as u8) - ('a' as u8) + 10),
        _ => Err(AnonymousRuntimeError::new("invalid hex digit".to_string())),
    }
}

pub fn decode_bytes(string: &String) -> Result<Vec<u8>, AnonymousRuntimeError> {
    let mut result: Vec<u8> = Vec::new();
    if string.len() % 2 != 0 {
        Err(AnonymousRuntimeError::new("hex word has odd length".to_string()))
    } else {
        let collected: Vec<char> = string.chars().collect();
        for i in 0..(collected.len() / 2) {
            let high: u8 = decode_letter(collected.get(i * 2).unwrap())?;
            let low: u8 = decode_letter(collected.get(i * 2 + 1).unwrap())?;
            result.push((high << 4) + low);
        }
        Ok(result)
    }
}

pub fn decode_string(string: &String) -> Result<Vec<u8>, AnonymousRuntimeError> {
    let mut result: Vec<u8> = Vec::new();
    for (i, character) in string.chars().enumerate() {
        if character > (255 as char) {
            return Err(AnonymousRuntimeError::new(format!("encountered invalid character in column {}", i)));
        } else {
            result.push(character as u8);
        }
    }
    Ok(result)
}