pub mod expansion;
pub mod scope;

use crate::block::Block;
use crate::error::EvaluationError;
use crate::evaluator::scope::EvaluatorScope;
use std::rc::Rc;

pub fn evaluate(
    blocks: &Vec<Rc<dyn Block>>,
    scope: &EvaluatorScope,
) -> Result<Vec<u8>, EvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    let mut inner = EvaluatorScope::child(scope);
    for block in blocks.iter() {
        result.extend(block.evaluate(&mut inner)?);
    }
    Ok(result)
}
