use std::collections::HashMap;

pub struct EvaluatorScope {
    level: usize,
    root: HashMap<String, Vec<u8>>,
    stack: Vec<HashMap<String, Vec<u8>>>,
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

    pub fn set(&mut self, name: String, value: Vec<u8>) {
        match self.stack.last_mut() {
            Some(map) => map.insert(name, value),
            None => self.root.insert(name, value),
        };
    }

    pub fn get(&self, name: &String) -> Option<&Vec<u8>> {
        for scope in self.stack.iter().rev() {
            match scope.get(name) {
                None => {},
                Some(result) => return Some(result),
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
