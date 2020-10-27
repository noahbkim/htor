use crate::evaluator::expansion::{Expansion, InlineExpansion};
use crate::error::{AnonymousEvaluationError, EvaluationError};
use crate::evaluator::Evaluator;
use crate::block::Block;
use std::rc::Rc;

pub struct DefineExpansion {
    name: String,
    parameters: Vec<String>,
    blocks: Vec<Rc<dyn Block>>,
}


impl DefineExpansion {
    pub fn new(name: String, parameters: Vec<String>, blocks: Vec<Rc<dyn Block>>) -> Self {
        Self { name, parameters, blocks }
    }
}

impl Expansion for DefineExpansion {
    fn expand(&self, evaluator: &mut Evaluator, args: &Vec<Vec<u8>>) -> Result<Vec<u8>, AnonymousEvaluationError> {
        if self.parameters.len() != args.len() {
            Err(AnonymousEvaluationError::new(format!("expansion ${} expected {} args, got {}", self.name, self.parameters.len(), args.len())))
        } else {
            evaluator.scope.push();
            for (name, value) in self.parameters.iter().zip(args) {
                evaluator.scope.set(&name, InlineExpansion::new(name.clone(), value.clone()));
            }
            let result: Vec<u8> = evaluator.evaluate(&self.blocks).map_err(|e|
                AnonymousEvaluationError::new(format!("error while expanding definition:\n{}", e)))?;
            evaluator.scope.pop();
            Ok(result)
        }
    }
}