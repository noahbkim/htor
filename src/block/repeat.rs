use crate::block::{Block, MacroBlock};
use crate::error::EvaluationError;
use crate::evaluator::Evaluator;

pub struct RepeatBlock {
    line_number: usize,
    repeat_count: usize,
    blocks: Vec<Box<dyn Block>>,
}

impl Block for RepeatBlock {
    fn evaluate(&self, evaluator: &mut Evaluator) -> Result<Vec<u8>, EvaluationError> {
        let mut result: Vec<u8> = Vec::new();
        for block in self.blocks.iter() {
            result.extend(block.evaluate(evaluator)?)
        }
        Ok(result)
    }
}

impl MacroBlock for RepeatBlock {
    fn new(line_number: usize, mut args: Vec<String>, blocks: Vec<Box<dyn Block>>) -> Result<Box<Self>, EvaluationError> {
        if args.len() == 1 {
            let arg: String = args.pop().unwrap();
            match arg.parse::<usize>() {
                Ok(repeat_count) => Ok(Box::new(Self { line_number, repeat_count, blocks })),
                Err(_) => Err(EvaluationError::new(line_number, format!("invalid repetition count {}", arg))),
            }
        } else {
            Err(EvaluationError::new(line_number, "expected exactly one argument indicating repetition count".to_string()))
        }
    }
}
