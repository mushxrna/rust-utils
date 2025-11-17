use crate::parser::*;
use std::collections::HashMap;

pub struct RefTable {
    reference_table: HashMap<Literal, Literal>,
}

impl RefTable {
    pub fn new() -> RefTable {
        RefTable {
            reference_table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: Literal, v: Literal) {
        self.reference_table.insert(k, v);
    }

    pub fn contains(&self, k: &Literal) -> bool {
        self.reference_table.contains_key(k)
    }

    pub fn retrieve(&self, k: &Literal) -> &Literal {
        &self.reference_table[k]
    }
}
