mod expansion;

use crate::block::define::expansion::DefineExpansion;
use crate::block::{Block, MacroBlock};
use crate::error::{AnonymousEvaluationError, EvaluationError};
use crate::evaluator::scope::EvaluatorScope;
use std::rc::Rc;

pub struct DefineBlock {
    line_number: usize,
    name: String,
    parameters: Vec<String>,
    blocks: Vec<Rc<dyn Block>>,
}

impl Block for DefineBlock {
    fn evaluate(&self, scope: &mut EvaluatorScope) -> Result<Vec<u8>, EvaluationError> {
        scope.set(
            &self.name,
            Box::new(DefineExpansion::new(
                self.name.clone(),
                self.parameters.clone(),
                self.blocks.clone(),
            )),
        );
        Ok(Vec::new())
    }
}

impl MacroBlock for DefineBlock {
    fn allocate(
        line_number: usize,
        mut args: Vec<String>,
        blocks: Vec<Rc<dyn Block>>,
    ) -> Result<Rc<Self>, EvaluationError> {
        if args.len() < 1 {
            Err(EvaluationError::new(
                line_number,
                "expected at least one argument indicating definition name".to_string(),
            ))
        } else {
            Ok(Rc::new(Self {
                line_number,
                name: args.remove(0),
                parameters: args,
                blocks,
            }))
        }
    }
}
