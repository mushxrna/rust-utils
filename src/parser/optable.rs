use std::collections::HashMap;

use crate::parser::Literal;

pub struct OpTable {
    pub table: HashMap<String, Box<dyn Fn(i32, i32) -> Literal>>,
}

impl OpTable {
    pub fn new() -> OpTable {
        OpTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, s: &str, func: Box<dyn Fn(i32, i32) -> Literal>) {
        self.table.insert(s.to_owned(), func);
    }

    pub fn contains(&self, string: &String) -> bool {
        self.table.contains_key(string)
    }
}
