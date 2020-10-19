use std::iter::once;

use super::Parser;
use super::decode::{decode_bytes, decode_string};
use super::error::ParserError;
use crate::parser::scope::ParserScope;
use crate::parser::error::AnonymousRuntimeError;


fn reverse_tail(vector: &mut Vec<u8>, from: usize) {
    let length = vector.len();
    for i in 0..(length - from) / 2 {
        vector.swap(from + i, length - 1 - i);
    }
}

enum BytesParserState {
    None,
    Bytes,
    String,
    StringEscaped,
    Name,
}

pub struct BytesParser<'a> {
    result: Vec<u8>,
    buffer: String,
    state: BytesParserState,
    scope: &'a ParserScope,
    flip: Option<usize>,
}

impl BytesParser {
    pub fn parse(line: &String, scope: &ParserScope) -> Result<Vec<u8>, AnonymousRuntimeError> {
        let parser = Self {
            result: Vec::new(),
            buffer: String::new(),
            state: BytesParserState::None,
            scope: &scope,
            flip: None,
        };
        parser.parse(line);
        Ok(parser.result)
    }

    fn set_flip(&mut self) {
        if let Some(start) = self.flip {
            reverse_tail(&mut self.result, *start);
        }
        *flip = Some(self.result.len());
    }

    fn unset_flip(&mut self) {
        if let Some(start) = self.flip {
            reverse_tail(&mut self.result, *start);
            *self.flip = None;
        }
    }

    fn terminate_bytes(&mut self) {
        self.result.extend(decode_bytes(&self.buffer)?);
        self.buffer.clear();
        self.state = BytesParserState::None;
    }

    fn terminate_string(&mut self) {
        self.result.extend(decode_string(&self.buffer)?);
        self.buffer.clear();
        self.state = BytesParserState::None;
    }

    fn terminate_name(&mut self) -> Result<(), AnonymousRuntimeError> {
        match self.scope.get(&self.buffer) {
            Some(value) => self.result.extend(value),
            None => return Err(AnonymousRuntimeError::new(format!("no definition for {}", name))),
        };
        self.buffer.clear();
        self.state = BytesParserState::None;
        Ok(())
    }

    fn terminate_escaped(&mut self, character: char) {
        self.result.push(character as u8);
        self.state = BytesParserState::String;
    }

    fn main(&mut self, line: &String) -> Result<(), AnonymousRuntimeError> {
        for character in line.chars().chain(once('\n')) {
            match state {
                BytesParserState::None => match character {
                    '"' => self.state = BytesParserState::String,
                    '$' => self.state = BytesParserState::Name,
                    '#' => break,
                    '>' => self.unset_flip(),
                    '<' => self.set_flip(),
                    '\t' | '\n' | '\x0C' | '\r' | ' ' => {}
                    _ => {
                        self.state = BytesParserState::Bytes;
                        self.buffer.push(character);
                    }
                },
                BytesParserState::Bytes => match character {
                    '\t' | '\n' | '\x0C' | '\r' | ' ' => self.terminate_bytes(),
                    '#' => {
                        self.terminate_bytes();
                        break;
                    }
                    '>' => {
                        self.terminate_bytes();
                        self.unset_flip();
                    },
                    '<' => {
                        self.terminate_bytes();
                        self.set_flip();
                    },
                    _ => self.buffer.push(character),
                },
                BytesParserState::String => match character {
                    '\\' => self.state = BytesParserState::StringEscaped,
                    '"' => self.terminate_string(),
                    _ => self.buffer.push(character),
                },
                BytesParserState::StringEscaped => match character {
                    '\\' => self.terminate_escaped('\\'),
                    'n' => self.terminate_escaped('\n'),
                    'r' => self.terminate_escaped('\r'),
                    't' => self.terminate_escaped('\t'),
                    '"' => self.terminate_escaped('"'),
                    _ => return Err(AnonymousRuntimeError::new(format!("invalid escape sequence \\{}", character)))
                }
                BytesParserState::Name => match character {
                    '\t' | '\n' | '\x0C' | '\r' | ' ' => self.terminate_name(),
                    '#' => {
                        self.terminate_name();
                        break;
                    }
                    _ => buffer.push(character)
                },
            }
        }

        self.unset_flip();
        if buffer.len() > 0 {
            Err(AnonymousRuntimeError::new("unexpected end of line".to_string()))
        } else {
            Ok(())
        }
    }
}
