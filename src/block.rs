pub mod assembly;
pub mod bytes;
pub mod repeat;
pub mod define;

use crate::error::EvaluationError;
use crate::evaluator::Evaluator;
use std::rc::Rc;

pub trait Block {
    fn evaluate(&self, evaluator: &mut Evaluator) -> Result<Vec<u8>, EvaluationError>;
}

pub trait MacroBlock {
    fn allocate(line_number: usize, args: Vec<String>, blocks: Vec<Rc<dyn Block>>) -> Result<Rc<Self>, EvaluationError>;
}

pub trait RawMacroBlock {
    fn allocate(line_number: usize, args: Vec<String>, lines: Vec<String>) -> Result<Rc<Self>, EvaluationError>;
}
