pub mod assembly;
pub mod bytes;
pub mod define;
pub mod repeat;

use crate::error::EvaluationError;
use crate::evaluator::scope::EvaluatorScope;
use std::rc::Rc;

pub trait Block {
    fn evaluate(&self, scope: &mut EvaluatorScope) -> Result<Vec<u8>, EvaluationError>;
}

pub trait MacroBlock {
    fn allocate(
        line_number: usize,
        args: Vec<String>,
        blocks: Vec<Rc<dyn Block>>,
    ) -> Result<Rc<Self>, EvaluationError>;
}

pub trait RawMacroBlock {
    fn allocate(
        line_number: usize,
        args: Vec<String>,
        lines: Vec<String>,
    ) -> Result<Rc<Self>, EvaluationError>;
}
