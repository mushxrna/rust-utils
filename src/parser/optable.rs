use std::collections::HashMap;

pub struct OpTable {
    table: HashMap<String, Box<dyn Fn()>>,
}

impl OpTable {
    pub fn new() -> OpTable {
        OpTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, s: &str, func: Box<dyn Fn()>) {
        self.table.insert(s.to_owned(), func);
    }

    pub fn contains(&self, string: &String) -> bool {
        self.table.contains_key(string)
    }
}
