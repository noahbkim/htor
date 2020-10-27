use crate::evaluator::Evaluator;
use crate::error::AnonymousEvaluationError;
use crate::block::Block;
use crate::block::bytes::BytesBlock;

pub trait Expansion {
    fn expand(&self, evaluator: &mut Evaluator, args: &Vec<Vec<u8>>) -> Result<Vec<u8>, AnonymousEvaluationError>;
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
    fn expand(&self, _: &mut Evaluator, args: &Vec<Vec<u8>>) -> Result<Vec<u8>, AnonymousEvaluationError> {
        if !args.is_empty() {
            Err(AnonymousEvaluationError::new(format!("expansion ${} expected 0 args, got {}", self.name, args.len())))
        } else {
            Ok(self.value.clone())
        }
    }
}
