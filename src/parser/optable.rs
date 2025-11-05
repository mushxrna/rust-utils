use std::collections::HashMap;

use crate::parser::{Literal, Operand};

pub struct OpTable {
    pub table: HashMap<String, Box<dyn Fn(Vec<Literal>) -> Literal>>,
    pub typetable: HashMap<String, Operand>,
}

impl OpTable {
    pub fn new() -> OpTable {
        OpTable {
            table: HashMap::new(),
            typetable: HashMap::new(),
        }
    }

    pub fn insert(&mut self, op: Operand, func: Box<dyn Fn(Vec<Literal>) -> Literal>) {
        self.table.insert(op.as_string().to_owned(), func);
        self.typetable.insert(op.as_string().to_owned(), op);
    }

    pub fn contains(&self, string: &String) -> bool {
        self.table.contains_key(string)
    }
}
