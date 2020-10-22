use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::{Enumerate};

use crate::parser::error::RuntimeError;

pub type EnumeratedLines = Enumerate<Lines<BufReader<File>>>;

pub struct ParserCursor {
    pub line: String,
    pub line_number: usize,
    lines: EnumeratedLines,
}

impl ParserCursor {
    pub fn new(lines: EnumeratedLines) -> Self {
        ParserCursor {
            line: String::new(),
            line_number: 0,
            lines,
        }
    }

    pub fn advance(&mut self) -> Result<bool, RuntimeError> {
        match self.lines.next() {
            None => Ok(false),
            Some((line_number, line_result)) => match line_result {
                Err(_) => Err(RuntimeError::new(line_number, "failed to read line".to_string())),
                Ok(line) => {
                    self.line = line;
                    self.line_number = line_number;
                    Ok(true)
                }
            },
        }
    }

    pub fn error(&self, what: String) -> RuntimeError {
        RuntimeError::new(self.line_number, what)
    }
}
