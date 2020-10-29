use crate::evaluator::expansion::Expansion;
use std::collections::HashMap;

type Link<'a> = Option<&'a EvaluatorScope<'a>>;

pub struct EvaluatorScope<'a> {
    expansions: HashMap<String, Box<dyn Expansion>>,
    parent: Link<'a>,
}

impl<'a> EvaluatorScope<'a> {
    pub fn new() -> Self {
        Self {
            expansions: HashMap::new(),
            parent: None,
        }
    }

    pub fn child(parent: &'a EvaluatorScope<'a>) -> Self {
        Self {
            expansions: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn set(&mut self, name: &String, expansion: Box<dyn Expansion>) {
        self.expansions.insert(name.clone(), expansion);
    }

    pub fn get(&self, name: &String) -> Option<&Box<dyn Expansion>> {
        if let Some(expansion) = self.expansions.get(name) {
            Some(expansion)
        } else {
            let mut cursor: &Link = &self.parent;
            while let Some(scope) = cursor {
                if let Some(expansion) = scope.expansions.get(name) {
                    return Some(expansion);
                }
                cursor = &scope.parent;
            }
            None
        }
    }
}
