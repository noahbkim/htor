mod error;
mod indentation;
mod cursor;
mod scope;
mod decode;
mod bytes;
mod assembly;

use std::fs::File;
use std::io::{BufReader, Lines, Write, BufRead};
use std::iter::{once, Enumerate};
use std::process::{Command, Stdio};

use error::ParserError;
use indentation::ParserIndentation;
use cursor::ParserCursor;
use scope::ParserScope;
use decode::{DecoderError, decode_bytes, decode_string};
use bytes::BytesParser;
use assembly::compile_assembly;


pub struct Parser {
    pub cursor: ParserCursor,
    pub indentation: ParserIndentation,
    pub scope: ParserScope,
}

impl Parser {
    pub fn new<T>(reader: BufReader<T>) -> Self {
        Self {
            cursor: ParserCursor::new(reader.lines().enumerate()),
            indentation: ParserIndentation::new(),
            scope: ParserScope::new(),
        }
    }

    fn parse_repeat(&mut self, args: Vec<&str>, level: usize) -> Result<(), ParserError> {
        match args.first() {
            None => Err(cursor.error("expected exactly one argument indicating repetition count".to_string())),
            Some(arg) => {
                if !cursor.advance()? {
                    return Ok(());
                }
                let count: usize = arg.parse::<usize>().map_err(|_e|
                    self.cursor.error(format!("invalid repetition count {}", arg)))?;
                let content: Vec<u8> = self.parse(level + 1);
                for _ in 0..count {
                    self.result.append();
                }
                Ok(())
            }
        }
    }


    pub fn parse(level: usize) -> Result<Vec<u8>, ParserError> {
        while on_indentation_level(cursor, context, level)? {
            cursor.line = String::from(cursor.line.trim());
            if cursor.line.starts_with("@") {
                let mut line: String = cursor.line.clone();
                line.find('#').map(|index| line.replace_range(index.., ""));

                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                let (head, tail) = words.split_at(1);
                match head.first() {
                    None => {
                        return Err(ParserError::new(
                            "macro lines must contain a command before the colon",
                            cursor.line_number,
                        ))
                    }
                    Some(&"@repeat") => {
                        result.extend(parse_repeat(tail.to_vec(), cursor, context, level)?)
                    }
                    Some(&"@define") => {
                        result.extend(parse_define(tail.to_vec(), cursor, context, level)?)
                    }
                    Some(&"@assembly") => {
                        result.extend(parse_assembly(tail.to_vec(), cursor, context, level)?)
                    }
                    Some(command) => {
                        return Err(ParserError::new(
                            format!("unknown command: {}", command).as_str(),
                            cursor.line_number,
                        ))
                    }
                };
            } else {
                result.extend(parse_bytes(cursor, context)?)
            }

            if !cursor.advance()? {
                break;
            }
        }
        Ok(result)
    }
}


fn parse_define(
    args: Vec<&str>,
    cursor: &mut ParserCursor,
    context: &mut ParserIndentation,
    level: usize,
) -> Result<Vec<u8>, ParserError> {
    match args.first() {
        None => {
            return Err(ParserError::new(
                "expected exactly one argument indicating definition name",
                cursor.line_number,
            ))
        }
        Some(arg) => {
            if !cursor.advance()? {
                return Ok(Vec::<u8>::new());
            }
            let name: String = arg.to_string();
            let contents: Vec<u8> = parse(cursor, context, level + 1)?;
            context.definitions.insert(name, contents);
        }
    }
    Ok(Vec::<u8>::new())
}

fn on_indentation_level(
    cursor: &ParserCursor,
    context: &mut ParserIndentation,
    level: usize,
) -> Result<bool, ParserError> {
    let indentation_level: usize = context.determine_level(cursor)?;
    if indentation_level < level {
        Ok(false)
    } else if indentation_level > level {
        Err(ParserError::new(
            format!(
                "expected indentation {}, found {}",
                level, indentation_level
            )
            .as_str(),
            cursor.line_number,
        ))
    } else {
        Ok(true)
    }
}

fn parse_raw(
    cursor: &mut ParserCursor,
    context: &mut ParserIndentation,
    level: usize,
) -> Result<String, ParserError> {
    let mut result: String = String::new();
    while on_indentation_level(cursor, context, level)? {
        result.push_str(cursor.line.trim_start());
        result.push('\n');
        if !cursor.advance()? {
            break;
        }
    }
    Ok(result)
}


fn parse_assembly(
    _args: Vec<&str>,
    cursor: &mut ParserCursor,
    context: &mut ParserIndentation,
    level: usize,
) -> Result<Vec<u8>, ParserError> {
    if !cursor.advance()? {
        Ok(Vec::<u8>::new())
    } else {
        let assembly = parse_raw(cursor, context, level + 1)?;
        compile_assembly(assembly)
    }
}


