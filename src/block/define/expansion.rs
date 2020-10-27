use crate::block::Block;
use crate::error::{AnonymousEvaluationError, EvaluationError};
use crate::evaluator::evaluate;
use crate::evaluator::expansion::{Expansion, InlineExpansion};
use crate::evaluator::scope::EvaluatorScope;
use std::rc::Rc;

pub struct DefineExpansion {
    name: String,
    parameters: Vec<String>,
    blocks: Vec<Rc<dyn Block>>,
}

impl DefineExpansion {
    pub fn new(name: String, parameters: Vec<String>, blocks: Vec<Rc<dyn Block>>) -> Self {
        Self {
            name,
            parameters,
            blocks,
        }
    }
}

impl Expansion for DefineExpansion {
    fn expand(
        &self,
        scope: &EvaluatorScope,
        args: &Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, AnonymousEvaluationError> {
        if self.parameters.len() != args.len() {
            Err(AnonymousEvaluationError::new(format!(
                "expansion ${} expected {} args, got {}",
                self.name,
                self.parameters.len(),
                args.len()
            )))
        } else {
            let mut inner: EvaluatorScope = EvaluatorScope::child(scope);
            for (name, value) in self.parameters.iter().zip(args) {
                inner.set(&name, InlineExpansion::new(name.clone(), value.clone()));
            }
            let result: Vec<u8> = evaluate(&self.blocks, &inner).map_err(|e| {
                AnonymousEvaluationError::new(format!("error while expanding definition:\n{}", e))
            })?;
            Ok(result)
        }
    }
}
