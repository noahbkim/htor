use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct ParserError {
    what: String,
}

impl ParserError {
    pub fn new(what: String) -> Self {
        Self { what }
    }
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}


pub struct AnonymousRuntimeError {
    what: String,
    column: usize,
}

impl AnonymousRuntimeError {
    pub fn new(what: String, column: usize) -> Self {
        Self { what, column }
    }

    pub fn at(self, line: usize, column_offset: usize) -> RuntimeError {
        RuntimeError {
            what: self.what,
            line,
            column: self.column + column_offset,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    what: String,
    line: usize,
    column: usize,
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}: {}", self.line, self.column, self.what)
    }
}
