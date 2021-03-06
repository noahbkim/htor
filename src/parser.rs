mod cursor;
mod indentation;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::error::{AnonymousEvaluationError, AnonymousEvaluationErrorResult};
use cursor::ParserCursor;
use indentation::ParserIndentation;

use crate::block::assembly::AssemblyBlock;
use crate::block::bytes::BytesBlock;
use crate::block::define::DefineBlock;
use crate::block::repeat::RepeatBlock;
use crate::block::{Block, MacroBlock, RawMacroBlock};
use crate::error::EvaluationError;
use std::rc::Rc;

pub struct Parser {
    cursor: ParserCursor,
    indentation: ParserIndentation,
}

fn tokenize_macro(mut line: String) -> Result<(String, Vec<String>), AnonymousEvaluationError> {
    line.find('#').map(|index| line.replace_range(index.., ""));
    let words: Vec<&str> = line.split_ascii_whitespace().collect();
    let (head, tail) = words.split_at(1);
    match head.first() {
        None => Err(AnonymousEvaluationError::new(
            "macro lines must contain a command before the colon".to_string(),
        )),
        Some(macro_name) => Ok((
            String::from(*macro_name),
            tail.iter().map(|s| String::from(*s)).collect(),
        )),
    }
}

impl Parser {
    pub fn new(reader: BufReader<File>) -> Result<Self, EvaluationError> {
        Ok(Self {
            cursor: ParserCursor::new(reader.lines()),
            indentation: ParserIndentation::new(),
        })
    }

    fn parse_raw(&mut self, level: usize) -> Result<Vec<String>, EvaluationError> {
        let mut result: Vec<String> = Vec::new();
        while self.cursor.advance()?
            && self
                .indentation
                .ge(&self.cursor.get_line(), level)
                .map_err_at(self.cursor.get_line_number())?
        {
            result.push(self.indentation.trim(&self.cursor.get_line(), level));
        }
        Ok(result)
    }

    fn parse(&mut self, level: usize) -> Result<Vec<Rc<dyn Block>>, EvaluationError> {
        let mut result: Vec<Rc<dyn Block>> = Vec::new();
        while self.cursor.advance()?
            && self
                .indentation
                .eq(&self.cursor.get_line(), level)
                .map_err_at(self.cursor.get_line_number())?
        {
            let line: String = String::from(self.cursor.get_line().trim());
            if line.starts_with("@") {
                let (macro_name, args): (String, Vec<String>) =
                    tokenize_macro(line).map_err_at(self.cursor.get_line_number())?;
                match macro_name.as_str() {
                    "@repeat" => result.push(RepeatBlock::allocate(
                        self.cursor.get_line_number(),
                        args,
                        self.parse(level + 1)?,
                    )?),
                    "@define" => result.push(DefineBlock::allocate(
                        self.cursor.get_line_number(),
                        args,
                        self.parse(level + 1)?,
                    )?),
                    "@assembly" => result.push(AssemblyBlock::allocate(
                        self.cursor.get_line_number(),
                        args,
                        self.parse_raw(level + 1)?,
                    )?),
                    _ => {
                        return Err(EvaluationError::new(
                            self.cursor.get_line_number(),
                            format!("unknown macro: {}", macro_name),
                        ))
                    }
                };
            } else {
                result.push(Rc::new(BytesBlock::new(
                    self.cursor.get_line_number(),
                    line,
                )?));
            }
        }
        Ok(result)
    }
}

pub fn parse(reader: BufReader<File>) -> Result<Vec<Rc<dyn Block>>, EvaluationError> {
    let mut parser: Parser = Parser::new(reader)?;
    parser.parse(0)
}
