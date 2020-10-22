pub(crate) mod error;
mod indentation;
mod cursor;
mod scope;
mod decode;
mod bytes;
mod assembly;

use std::fs::File;
use std::io::{BufReader, BufRead};

use error::ParserError;
use indentation::ParserIndentation;
use cursor::ParserCursor;
use scope::ParserScope;
use bytes::BytesParser;
use assembly::compile_assembly;
use crate::parser::error::RuntimeError;


pub struct Parser {
    pub cursor: ParserCursor,
    pub indentation: ParserIndentation,
    pub scope: ParserScope,
}

impl Parser {
    pub fn new(reader: BufReader<File>) -> Self {
        Self {
            cursor: ParserCursor::new(reader.lines().enumerate()),
            indentation: ParserIndentation::new(),
            scope: ParserScope::new(),
        }
    }

    fn on_indentation_level(&mut self, level: usize) -> Result<bool, RuntimeError> {
        let indentation_level: usize = context.determine_level(cursor)?;
        if indentation_level < level {
            Ok(false)
        } else if indentation_level > level {
            Err(self.cursor.error(format!("expected indentation {}, found {}", level, indentation_level)))
        } else {
            Ok(true)
        }
    }

    fn parse_repeat(&mut self, args: Vec<&str>, level: usize) -> Result<Vec<u8>, RuntimeError> {
        match args.first() {
            None => Err(cursor.error("expected exactly one argument indicating repetition count".to_string())),
            Some(arg) => {
                if !cursor.advance()? {
                    return Ok(Vec::new());
                }
                let count: usize = arg.parse::<usize>().map_err(|_e| self.cursor.error(format!("invalid repetition count {}", arg)))?;
                let content: Vec<u8> = self.parse(level + 1)?;
                Ok(content.repeat(count))
            }
        }
    }

    fn parse_define(&mut self, args: Vec<&str>, level: usize) -> Result<Vec<u8>, RuntimeError> {
        match args.first() {
            None => return Err(self.cursor.error("expected exactly one argument indicating definition name".to_string())),
            Some(arg) => {
                if !cursor.advance()? {
                    return Ok(Vec::<u8>::new());
                }
                let name: String = arg.to_string();
                let contents: Vec<u8> = self.parse(level + 1)?;
                context.definitions.insert(name, contents);
            }
        }
        Ok(Vec::<u8>::new())
    }

    fn parse_raw(&mut self, level: usize) -> Result<String, RuntimeError> {
        let mut result: String = String::new();
        while self.on_indentation_level(level)? {
            result.push_str(cursor.line.trim_start());
            result.push('\n');
            if !cursor.advance()? {
                break;
            }
        }
        Ok(result)
    }

    fn parse_assembly(&mut self, _args: Vec<&str>, level: usize) -> Result<Vec<u8>, RuntimeError> {
        if !self.cursor.advance()? {
            Ok(Vec::<u8>::new())
        } else {
            let line_number = self.cursor.line_number;
            let assembly = self.parse_raw(level + 1)?;
            compile_assembly(&assembly).map_err(|e| e.at(line_number))
        }
    }

    pub fn parse(&mut self, level: usize) -> Result<Vec<u8>, RuntimeError> {
        let mut result: Vec<u8> = Vec::new();
        while on_indentation_level(cursor, context, level)? {
            self.cursor.line = String::from(cursor.line.trim());
            if self.cursor.line.starts_with("@") {
                let mut line: String = cursor.line.clone();
                line.find('#').map(|index| line.replace_range(index.., ""));

                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                let (head, tail) = words.split_at(1);
                match head.first() {
                    None => {
                        return Err(self.cursor.error("macro lines must contain a command before the colon".to_string()))
                    }
                    Some(&"@repeat") => {
                        result.extend(self.parse_repeat(tail.to_vec(), level)?)
                    }
                    Some(&"@define") => {
                        result.extend(self.parse_define(tail.to_vec(), level)?)
                    }
                    Some(&"@assembly") => {
                        result.extend(self.parse_assembly(tail.to_vec(), level)?)
                    }
                    Some(command) => {
                        return Err(self.cursor.error(format!("unknown command: {}", command)))
                    }
                };
            } else {
                result.extend(BytesParser::parse(&self.cursor.line, &self.scope).map_err(|e| e.at(self.cursor.line_number))?);
            }

            if !cursor.advance()? {
                break;
            }
        }
        Ok(result)
    }
}


