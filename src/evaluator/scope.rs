use std::collections::HashMap;
use std::ops::Deref;

pub struct EvaluatorScope {
    root: HashMap<String, Vec<u8>>,
    stack: Vec<HashMap<String, Vec<u8>>>,
}

impl EvaluatorScope {
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn set(&mut self, name: String, value: Vec<u8>) {
        match self.stack.last_mut() {
            Some(map) => map.insert(name, value),
            None => self.root.insert(name, value),
        };
    }

    pub fn get(&self, name: &String) -> Option<Vec<u8>> {
        for scope in self.stack.iter().rev() {
            match scope.get(name) {
                None => {},
                Some(result) => return Some(result.clone()),
            };
        }
        None
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
