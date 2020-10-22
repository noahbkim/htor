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
}

impl AnonymousRuntimeError {
    pub fn new(what: String) -> Self {
        Self { what }
    }

    pub fn at(self, line: usize) -> RuntimeError {
        RuntimeError::new(line, self.what)
    }
}


#[derive(Debug)]
pub struct RuntimeError {
    what: String,
    line: usize,
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error on line {}: {}", self.line, self.what)
    }
}

impl RuntimeError {
    pub fn new(line: usize, what: String) -> Self {
        RuntimeError { what, line }
    }
}
