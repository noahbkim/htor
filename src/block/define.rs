mod expansion;

use std::rc::Rc;
use crate::block::{Block, MacroBlock};
use crate::error::{EvaluationError, AnonymousEvaluationError};
use crate::evaluator::Evaluator;
use crate::block::define::expansion::DefineExpansion;


pub struct DefineBlock {
    line_number: usize,
    name: String,
    parameters: Vec<String>,
    blocks: Vec<Rc<dyn Block>>,
}

impl Block for DefineBlock {
    fn evaluate(&self, evaluator: &mut Evaluator) -> Result<Vec<u8>, EvaluationError> {
        evaluator.scope.set(&self.name, Box::new(DefineExpansion::new(
            self.name.clone(),
            self.parameters.clone(),
            self.blocks.clone(),
        )));
        Ok(Vec::new())
    }
}

impl MacroBlock for DefineBlock {
    fn allocate(line_number: usize, mut args: Vec<String>, blocks: Vec<Rc<dyn Block>>) -> Result<Rc<Self>, EvaluationError> {
        if args.len() < 1 {
            Err(EvaluationError::new(line_number, "expected at least one argument indicating definition name".to_string()))
        } else {
            Ok(Rc::new(Self {
                line_number,
                name: args.pop().unwrap(),
                parameters: args,
                blocks
            }))
        }
    }
}
