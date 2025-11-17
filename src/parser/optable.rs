use std::collections::HashMap;

use crate::parser::{Literal, Operand};

pub struct OpTable {
    pub function_table: HashMap<String, Box<dyn Fn(&Vec<&Literal>) -> Literal>>,
    pub operand_table: HashMap<String, Operand>,
}

impl OpTable {
    pub fn new() -> OpTable {
        OpTable {
            function_table: HashMap::new(),
            operand_table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, op: Operand, func: Box<dyn Fn(&Vec<&Literal>) -> Literal>) {
        self.function_table.insert(op.as_string().to_owned(), func);
        self.operand_table.insert(op.as_string().to_owned(), op);
    }

    pub fn contains(&self, string: &String) -> bool {
        self.operand_table.contains_key(string)
    }

    pub fn call_by_operand(&self, op: &Operand, args: &Vec<&Literal>) -> Literal {
        self.function_table[op.as_string()](args)
    }

    pub fn insert_binary_op(&mut self, str: &str, func: fn(&Vec<&Literal>) -> Literal) {
        let op = Operand::Binary(str.to_owned());
        self.insert(op, Box::new(func))
    }

    pub fn insert_dropin_op(&mut self, str: &str, replace: String) {
        let op = Operand::DropIn(str.to_owned());
        self.insert(
            op,
            Box::new(move |l| -> Literal { Literal::Word(replace.clone()) }),
        )
    }

    pub fn insert_function_op(&mut self, str: &str, func: fn(&Vec<&Literal>) -> Literal) {
        let op = Operand::Function(str.to_owned());
        self.insert(op, Box::new(func))
    }

    pub fn insert_assignment_op(&mut self, str: &str, func: fn(&Vec<&Literal>) -> Literal) {
        let op = Operand::Assignment(str.to_owned());
        self.insert(op, Box::new(func))
    }
}
