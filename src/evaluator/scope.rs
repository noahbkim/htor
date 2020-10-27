use std::collections::HashMap;
use crate::block::Block;
use crate::block::define::DefineBlock;
use crate::evaluator::expansion::Expansion;

pub struct EvaluatorScope {
    level: usize,
    root: HashMap<String, Box<dyn Expansion>>,
    stack: Vec<HashMap<String, Box<dyn Expansion>>>,
}

impl EvaluatorScope {
    pub fn new() -> Self {
        Self {
            level: 0,
            root: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        if self.level != 0 {
            self.stack.push(HashMap::new());
        }
        self.level += 1;
    }

    pub fn set(&mut self, name: &String, expansion: Box<dyn Expansion>) {
        match self.stack.last_mut() {
            Some(map) => map.insert(name.clone(), expansion),
            None => self.root.insert(name.clone(), expansion),
        };
    }

    pub fn get(&self, name: &String) -> Option<&Box<dyn Expansion>> {
        for scope in self.stack.iter().rev() {
            match scope.get(name) {
                None => {},
                Some(block) => return Some(block),
            };
        }
        self.root.get(name)
    }

    pub fn pop(&mut self) {
        if self.level > 0 {
            self.stack.pop();
        }
        self.level -= 1;
    }
}
