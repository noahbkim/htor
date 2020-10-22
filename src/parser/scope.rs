use std::collections::HashMap;

pub struct ParserScope {
    definitions: Vec<HashMap<String, Vec<u8>>>,
}

impl ParserScope {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.definitions.push(HashMap::new());
    }

    pub fn get(&self, name: &String) -> Option<Vec<u8>> {
        for scope in self.definitions.iter().rev() {
            match scope.get(name) {
                None => {},
                Some(result) => return Some(result.clone()),
            };
        }
        None
    }

    pub fn pop(&mut self) {
        self.definitions.pop();
    }
}
