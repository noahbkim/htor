pub mod assembly;
pub mod bytes;
pub mod repeat;
pub mod define;

use crate::error::EvaluationError;
use crate::evaluator::Evaluator;

pub trait Block {
    fn evaluate(&self, parser: &mut Evaluator) -> Result<Vec<u8>, EvaluationError>;
}

pub trait MacroBlock {
    fn new(line_number: usize, args: Vec<String>, blocks: Vec<Box<dyn Block>>) -> Result<Box<Self>, EvaluationError>;
}

pub trait RawMacroBlock {
    fn new(line_number: usize, args: Vec<String>, lines: Vec<String>) -> Result<Box<Self>, EvaluationError>;
}
