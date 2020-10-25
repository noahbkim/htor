use super::Block;

use std::iter::once;

use crate::error::AnonymousEvaluationError;
use crate::error::EvaluationError;
use crate::evaluator::Evaluator;

enum BytesItem {
    Bytes(Vec<u8>),
    Name(String),
    Left,
    Right,
}

enum BytesState {
    None,
    Bytes,
    String,
    StringEscaped,
    Name,
}

fn decode_letter(letter: &char) -> Result<u8, AnonymousEvaluationError> {
    match letter {
        '0'..='9' => Ok((*letter as u8) - ('0' as u8)),
        'A'..='F' => Ok((*letter as u8) - ('A' as u8) + 10),
        'a'..='f' => Ok((*letter as u8) - ('a' as u8) + 10),
        _ => Err(AnonymousEvaluationError::new("invalid hex digit".to_string())),
    }
}

fn decode_bytes(string: &String) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    if string.len() % 2 != 0 {
        Err(AnonymousEvaluationError::new("hex word has odd length".to_string()))
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

fn decode_string(string: &String) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    for (i, character) in string.chars().enumerate() {
        if character > (255 as char) {
            return Err(AnonymousEvaluationError::new(format!("encountered invalid character in column {}", i)));
        } else {
            result.push(character as u8);
        }
    }
    Ok(result)
}

pub struct BytesBlock {
    line_number: usize,
    items: Vec<BytesItem>,
}

impl BytesBlock {
    pub fn new(line_number: usize, line: &String) -> Result<Box<Self>, EvaluationError> {
        let mut items: Vec<BytesItem> = Vec::new();
        let mut state: BytesState = BytesState::None;
        let mut buffer: String = String::new();

        for character in line.chars().chain(once('\n')) {
            match state {
                BytesState::None => match character {
                    '"' => state = BytesState::String,
                    '$' => state = BytesState::Name,
                    '#' => break,
                    '>' => items.push(BytesItem::Right),
                    '<' => items.push(BytesItem::Left),
                    '\t' | '\n' | '\x0C' | '\r' | ' ' => {}
                    _ => {
                        state = BytesState::Bytes;
                        buffer.push(character);
                    }
                },
                BytesState::Bytes => match character {
                    '\t' | '\n' | '\x0C' | '\r' | ' ' | '#' | '>' | '<' => {
                        items.push(BytesItem::Bytes(decode_bytes(&buffer).map_err(|e| e.at(line_number))?));
                        buffer.clear();
                        state = BytesState::None;
                        match character {
                            '#' => break,
                            '>' => items.push(BytesItem::Right),
                            '<' => items.push(BytesItem::Left),
                            _ => {},
                        }
                    },
                    _ => buffer.push(character),
                },
                BytesState::String => match character {
                    '\\' => state = BytesState::StringEscaped,
                    '"' => {
                        items.push(BytesItem::Bytes(decode_string(&buffer).map_err(|e| e.at(line_number))?));
                        buffer.clear();
                        state = BytesState::None;
                    }
                    _ => buffer.push(character),
                },
                BytesState::StringEscaped => match character {
                    '\\' => {
                        buffer.push('\\');
                        state = BytesState::String;
                    }
                    'n' => {
                        buffer.push('\n');
                        state = BytesState::String;
                    }
                    'r' => {
                        buffer.push('\r');
                        state = BytesState::String;
                    }
                    't' => {
                        buffer.push('\t');
                        state = BytesState::String;
                    }
                    '"' => {
                        buffer.push('\"');
                        state = BytesState::String;
                    }
                    _ => return Err(EvaluationError::new(line_number, format!("invalid escape sequence \\{}", character)))
                }
                BytesState::Name => match character {
                    '\t' | '\n' | '\x0C' | '\r' | ' ' | '#' => {
                        items.push(BytesItem::Name(buffer.clone()));
                        buffer.clear();
                        state = BytesState::None;
                        if character == '#' {
                            break
                        }
                    }
                    _ => buffer.push(character)
                },
            }
        }

        if buffer.len() > 0 {
            Err(EvaluationError::new(line_number, "unexpected end of line".to_string()))
        } else {
            Ok(Box::new(BytesBlock { line_number, items }))
        }
    }
}

fn reverse_tail(vector: &mut Vec<u8>, from: usize) {
    let length = vector.len();
    for i in 0..(length - from) / 2 {
        vector.swap(from + i, length - 1 - i);
    }
}

impl Block for BytesBlock {
    fn evaluate(&self, evaluator: &mut Evaluator) -> Result<Vec<u8>, EvaluationError> {
        let mut result: Vec<u8> = Vec::new();
        let mut flip: Option<usize> = None;
        for item in self.items.iter() {
            match item {
                BytesItem::Left => {
                    if let Some(start) = flip {
                        reverse_tail(&mut result, start);
                    }
                    flip = Some(result.len());
                }
                BytesItem::Right => {
                    if let Some(start) = flip {
                        reverse_tail(&mut result, start);
                        flip = None;
                    }
                }
                BytesItem::Bytes(bytes) => {
                    result.extend(bytes);
                }
                BytesItem::Name(name) => {
                    result.extend(evaluator.scope.get(name).ok_or(
                        EvaluationError::new(self.line_number, format!("undefined variable {}", name)))?);
                }
            }
        }
        if let Some(start) = flip {
            reverse_tail(&mut result, start);
        }
        Ok(result)
    }
}
