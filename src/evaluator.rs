mod scope;
pub mod expansion;

use scope::EvaluatorScope;
use crate::block::Block;
use crate::error::EvaluationError;
use std::rc::Rc;

pub struct Evaluator {
    pub scope: EvaluatorScope,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { scope: EvaluatorScope::new() }
    }

    pub fn evaluate(&mut self, blocks: &Vec<Rc<dyn Block>>) -> Result<Vec<u8>, EvaluationError> {
        let mut result: Vec<u8> = Vec::new();
        self.scope.push();
        for block in blocks.iter() {
            result.extend(block.evaluate(self)?);
        }
        self.scope.pop();
        Ok(result)
    }
}

pub fn evaluate(blocks: &Vec<Rc<dyn Block>>) -> Result<Vec<u8>, EvaluationError> {
    let mut evaluator: Evaluator = Evaluator::new();
    evaluator.evaluate(blocks)
}