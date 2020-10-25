use crate::block::{Block, MacroBlock};
use crate::error::EvaluationError;
use crate::evaluator::Evaluator;

pub struct DefineBlock {
    line_number: usize,
    name: String,
    blocks: Vec<Box<dyn Block>>,
}

impl Block for DefineBlock {
    fn evaluate(&self, evaluator: &mut Evaluator) -> Result<Vec<u8>, EvaluationError> {
        let mut result: Vec<u8> = Vec::new();
        for block in self.blocks.iter() {
            result.extend(block.evaluate(evaluator)?)
        }
        evaluator.scope.set(self.name.clone(), result);
        Ok(Vec::new())
    }
}

impl MacroBlock for DefineBlock {
    fn new(line_number: usize, mut args: Vec<String>, blocks: Vec<Box<dyn Block>>) -> Result<Box<Self>, EvaluationError> {
        if args.len() == 1 {
            Ok(Box::new(Self { line_number, name: args.pop().unwrap(), blocks }))
        } else {
            Err(EvaluationError::new(line_number, "expected exactly one argument indicating definition name".to_string()))
        }
    }
}
