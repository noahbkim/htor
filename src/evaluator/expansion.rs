use crate::error::AnonymousEvaluationError;
use crate::evaluator::scope::EvaluatorScope;

pub trait Expansion {
    fn expand(
        &self,
        scope: &EvaluatorScope,
        args: &Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, AnonymousEvaluationError>;
}

pub struct InlineExpansion {
    name: String,
    value: Vec<u8>,
}

impl InlineExpansion {
    pub fn new(name: String, value: Vec<u8>) -> Box<Self> {
        Box::new(Self { name, value })
    }
}

impl Expansion for InlineExpansion {
    fn expand(
        &self,
        _: &EvaluatorScope,
        args: &Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, AnonymousEvaluationError> {
        if !args.is_empty() {
            Err(AnonymousEvaluationError::new(format!(
                "expansion ${} expected 0 args, got {}",
                self.name,
                args.len()
            )))
        } else {
            Ok(self.value.clone())
        }
    }
}
