use std::fs::File;
use std::io::{BufReader, Lines};

use crate::error::EvaluationError;

pub struct ParserCursor {
    line: String,
    line_number: usize,
    lines: Lines<BufReader<File>>,
}

impl ParserCursor {
    pub fn new(lines: Lines<BufReader<File>>) -> Self {
        ParserCursor {
            line: String::new(),
            line_number: 0,
            lines,
        }
    }

    pub fn advance(&mut self) -> Result<bool, EvaluationError> {
        match self.lines.next() {
            None => Ok(false),
            Some(line) => match line {
                Err(_) => Err(EvaluationError::new(
                    self.line_number,
                    "failed to read line".to_string(),
                )),
                Ok(line) => {
                    self.line = line;
                    self.line_number += 1;
                    Ok(true)
                }
            },
        }
    }

    pub fn get_line(&self) -> &String {
        return &self.line;
    }

    pub fn get_line_number(&self) -> usize {
        return self.line_number;
    }
}
