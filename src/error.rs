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


pub struct AnonymousEvaluationError {
    what: String,
}

impl AnonymousEvaluationError {
    pub fn new(what: String) -> Self {
        Self { what }
    }
    pub fn at(self, line: usize) -> EvaluationError {
        EvaluationError::new(line, self.what)
    }
}

pub trait AnonymousEvaluationErrorResult<T> {
    fn map_err_at(self, line_number: usize) -> Result<T, EvaluationError>;
}

impl<T> AnonymousEvaluationErrorResult<T> for Result<T, AnonymousEvaluationError> {
    fn map_err_at(self, line_number: usize) -> Result<T, EvaluationError> {
        self.map_err(|e| e.at(line_number))
    }
}


#[derive(Debug)]
pub struct EvaluationError {
    what: String,
    line: usize,
}

impl Error for EvaluationError {}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error on line {}: {}", self.line, self.what)
    }
}

impl EvaluationError {
    pub fn new(line: usize, what: String) -> Self {
        EvaluationError { what, line }
    }
}
