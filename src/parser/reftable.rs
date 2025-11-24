use crate::parser::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RefTable {
    reference_table: RefCell<HashMap<Literal, Literal>>,
}

impl RefTable {
    pub fn new() -> RefTable {
        RefTable {
            reference_table: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, k: Literal, v: Literal) {
        self.reference_table.borrow_mut().insert(k, v);
    }

    pub fn contains(&self, k: &Literal) -> bool {
        self.reference_table.borrow().contains_key(k)
    }

    pub fn retrieve(&self, k: &Literal) -> Option<Literal> {
        if self.contains(k) {
            Some(self.reference_table.borrow()[k].clone())
        } else {
            None
        }
    }
}
