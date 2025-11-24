use std::cell::RefCell;
use std::collections::HashMap;

use crate::parser::{Literal, Operand};

pub struct OpTable {
    pub function_table:
        RefCell<HashMap<String, Box<dyn Fn(&Vec<&Literal>) -> Result<Literal, String>>>>,
    pub operand_table: RefCell<HashMap<String, Operand>>,
}

impl OpTable {
    pub fn new() -> OpTable {
        OpTable {
            function_table: RefCell::new(HashMap::new()),
            operand_table: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(
        &self,
        op: Operand,
        func: Box<dyn Fn(&Vec<&Literal>) -> Result<Literal, String>>,
    ) {
        self.function_table
            .borrow_mut()
            .insert(op.ref_string().to_owned(), func);
        self.operand_table
            .borrow_mut()
            .insert(op.ref_string().to_owned(), op);
    }

    pub fn contains(&self, string: &String) -> bool {
        self.operand_table.borrow().contains_key(string)
    }

    pub fn call_by_operand(&self, op: &Operand, args: &Vec<&Literal>) -> Result<Literal, String> {
        self.function_table.borrow()[op.ref_string()](args)
    }

    pub fn insert_binary_op(&self, str: &str, func: fn(&Vec<&Literal>) -> Result<Literal, String>) {
        let op = Operand::Binary(str.to_owned());
        self.insert(op, Box::new(func))
    }

    pub fn insert_dropin_op(&self, str: &str, replace: Literal) {
        let op = Operand::DropIn(str.to_owned());
        self.insert(
            op,
            Box::new(move |l| -> Result<Literal, String> { Ok(replace.clone()) }),
        )
    }

    pub fn insert_function_op(
        &self,
        str: &str,
        func: fn(&Vec<&Literal>) -> Result<Literal, String>,
    ) {
        let op = Operand::Function(str.to_owned());
        self.insert(op, Box::new(func))
    }

    pub fn insert_assignment_op(&mut self, str: &str) {
        let op = Operand::Assignment(str.to_owned());
        self.insert(
            op,
            Box::new(|v| -> Result<Literal, String> { Ok(Literal::Word("".to_string())) }),
        )
    }
}
