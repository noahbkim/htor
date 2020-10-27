use crate::error::AnonymousEvaluationError;

pub fn byte_from_hexadecimal_digit(digit: char) -> Result<u8, AnonymousEvaluationError> {
    match digit {
        '0'..='9' => Ok((digit as u8) - ('0' as u8)),
        'A'..='F' => Ok((digit as u8) - ('A' as u8) + 10),
        'a'..='f' => Ok((digit as u8) - ('a' as u8) + 10),
        _ => Err(AnonymousEvaluationError::new(
            "invalid hexadecimal digit".to_string(),
        )),
    }
}

pub fn integer_from_hexadecimal(string: &str) -> Result<usize, AnonymousEvaluationError> {
    let mut result: usize = 0;
    for digit in string.chars() {
        result <<= 4;
        result += byte_from_hexadecimal_digit(digit)? as usize;
    }
    Ok(result)
}

pub fn bytes_from_hexadecimal(
    string: &str,
    strict: bool,
) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    let mut collected: Vec<char> = string.chars().collect();

    if string.len() % 2 != 0 {
        if strict {
            return Err(AnonymousEvaluationError::new(
                "length of hexadecimal word must be divisible by two".to_string(),
            ));
        } else {
            collected.insert(0, '0');
        }
    }

    for i in 0..(collected.len() / 2) {
        let high: u8 = byte_from_hexadecimal_digit(*collected.get(i * 2).unwrap())?;
        let low: u8 = byte_from_hexadecimal_digit(*collected.get(i * 2 + 1).unwrap())?;
        result.push((high << 4) + low);
    }
    Ok(result)
}

pub fn integer_from_decimal(string: &str) -> Result<usize, AnonymousEvaluationError> {
    string
        .parse::<usize>()
        .map_err(|e| AnonymousEvaluationError::new(format!("invalid decimal format: {}", e)))
}

pub fn bytes_from_decimal(string: &str) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    let mut work: usize = integer_from_decimal(string)?;
    while work > 0 {
        result.push((work % 256) as u8);
        work = work / 256;
    }
    result.reverse();
    Ok(result)
}

pub fn byte_from_binary_digit(digit: char) -> Result<u8, AnonymousEvaluationError> {
    match digit {
        '0'..='1' => Ok((digit as u8) - ('0' as u8)),
        _ => Err(AnonymousEvaluationError::new(
            "invalid binary digit".to_string(),
        )),
    }
}

pub fn integer_from_binary(string: &str) -> Result<usize, AnonymousEvaluationError> {
    let mut result: usize = 0;
    for digit in string.chars() {
        result <<= 1;
        result += byte_from_binary_digit(digit)? as usize;
    }
    Ok(result)
}

pub fn bytes_from_binary(string: &str, strict: bool) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    let mut collected: Vec<char> = string.chars().collect();

    if string.len() % 8 != 0 {
        if strict {
            return Err(AnonymousEvaluationError::new(
                "length of binary word must be divisible by eight".to_string(),
            ));
        } else {
            for _ in 0..(8 - string.len() % 8) {
                collected.insert(0, '0')
            }
        }
    }

    for i in 0..(collected.len() / 8) {
        let mut byte: u8 = 0;
        for j in 0..8 {
            byte <<= 1;
            byte += byte_from_binary_digit(*collected.get(i + j).unwrap())?;
        }
        result.push(byte);
    }
    Ok(result)
}

pub fn bytes_from_number(string: &str, strict: bool) -> Result<Vec<u8>, AnonymousEvaluationError> {
    if string.starts_with("0x") {
        bytes_from_hexadecimal(&string[2..], strict)
    } else if string.starts_with("0d") {
        bytes_from_decimal(&string[2..])
    } else if string.starts_with("0b") {
        bytes_from_binary(&string[2..], strict)
    } else {
        bytes_from_hexadecimal(&string, strict)
    }
}

pub fn integer_from_number(string: &str) -> Result<usize, AnonymousEvaluationError> {
    if string.starts_with("0x") {
        integer_from_hexadecimal(&string[2..])
    } else if string.starts_with("0d") {
        integer_from_decimal(&string[2..])
    } else if string.starts_with("0b") {
        integer_from_binary(&string[2..])
    } else {
        integer_from_hexadecimal(&string)
    }
}
