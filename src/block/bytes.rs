mod parser;
mod translate;

use super::Block;
use crate::block::bytes::parser::{parse_bytes, BytesItem};
use crate::error::AnonymousEvaluationErrorResult;
use crate::error::EvaluationError;
use crate::evaluator::expansion::Expansion;
use crate::evaluator::scope::EvaluatorScope;

pub struct BytesBlock {
    line_number: usize,
    items: Vec<BytesItem>,
}

impl BytesBlock {
    pub fn new(line_number: usize, line: String) -> Result<Self, EvaluationError> {
        Ok(BytesBlock {
            line_number,
            items: parse_bytes(line.as_str()).map_err_at(line_number)?,
        })
    }
}

fn reverse_tail(vector: &mut Vec<u8>, from: usize) {
    let length = vector.len();
    for i in 0..(length - from) / 2 {
        vector.swap(from + i, length - 1 - i);
    }
}

fn evaluate(
    line_number: usize,
    items: &Vec<BytesItem>,
    scope: &mut EvaluatorScope,
) -> Result<Vec<u8>, EvaluationError> {
    let mut result: Vec<u8> = Vec::new();
    let mut flip: Option<usize> = None;
    for item in items.iter() {
        match item {
            BytesItem::Left => {
                if let Some(start) = flip {
                    reverse_tail(&mut result, start);
                }
                flip = Some(result.len());
            }
            BytesItem::Right => {
                if let Some(start) = flip {
                    reverse_tail(&mut result, start);
                    flip = None;
                }
            }
            BytesItem::Literal(bytes) => {
                result.extend(bytes);
            }
            BytesItem::Expansion(name, args) => {
                let mut expansion_args: Vec<Vec<u8>> = Vec::new();
                for arg in args {
                    expansion_args.push(evaluate(line_number, arg, scope)?);
                }

                let expansion: &Box<dyn Expansion> = scope.get(&name).ok_or(
                    EvaluationError::new(line_number, format!("undefined variable {}", name)),
                )?;
                result.extend(
                    expansion
                        .expand(scope, &expansion_args)
                        .map_err_at(line_number)?,
                );
            }
        }
    }
    if let Some(start) = flip {
        reverse_tail(&mut result, start);
    }
    Ok(result)
}

impl Block for BytesBlock {
    fn evaluate(&self, scope: &mut EvaluatorScope) -> Result<Vec<u8>, EvaluationError> {
        evaluate(self.line_number, &self.items, scope)
    }
}
