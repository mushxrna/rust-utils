use crate::parser::{BytePointer, errors::InternalError, heap::BytePtr};
use std::{borrow::Cow, hash::Hasher, marker::PhantomData};

pub struct Expression(pub Vec<Literal>);
impl Expression {
    pub fn inner(self) -> Vec<Literal> {
        self.0
    }
}

#[derive(Debug)]
pub enum Literal {
    Word(String),
    Operator(Operand),
    Pointer(Box<dyn BytePtr>),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Operand {
    Binary(String),
    DropIn(String),
    Function(String),
    Assignment(String),
}

impl Operand {
    pub fn ref_string(&self) -> &String {
        match self {
            Operand::Binary(op) => op,
            Operand::DropIn(op) => op,
            Operand::Function(op) => op,
            Operand::Assignment(op) => op,
        }
    }
}

impl Clone for Operand {
    fn clone(&self) -> Self {
        println!("Cloning Operand.");
        match self {
            Operand::DropIn(s) => Operand::DropIn(self.ref_string().clone()),
            Operand::Binary(s) => Operand::Binary(self.ref_string().clone()),
            Operand::Function(s) => Operand::Function(self.ref_string().clone()),
            Operand::Assignment(s) => Operand::Assignment(self.ref_string().clone()),
        }
    }
}

impl Literal {
    pub fn as_string(&self) -> String {
        match self {
            Literal::Word(string) => string.clone(),
            Literal::Operator(op) => op.ref_string().clone(),
            Literal::Pointer(p) => p.as_raw_ptr().to_string(),
        }
    }

    pub fn ref_string(&self) -> Result<&String, InternalError> {
        match self {
            Literal::Word(string) => Ok(string),
            Literal::Operator(op) => Ok(op.ref_string()),
            Literal::Pointer(p) => Err(InternalError::CannotReferencePtrStr),
        }
    }

    pub fn as_cow(&self) -> Cow<'_, str> {
        match self {
            Literal::Word(string) => Cow::Borrowed(string),
            Literal::Operator(op) => Cow::Borrowed(op.ref_string()),
            Literal::Pointer(p) => Cow::Owned(p.as_raw_ptr().to_string()),
        }
    }
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        println!("Cloning Literal.");
        match self {
            Literal::Word(s) => Literal::Word(s.clone()),
            Literal::Operator(o) => Literal::Operator(o.clone()),
            Literal::Pointer(p) => {
                let x = p.as_raw_ptr();
                let y: BytePointer<i32> = BytePointer::from_raw_ptr(x);
                Literal::Pointer(Box::new(y))
            }
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
    }
}

impl Eq for Literal {}

impl std::hash::Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Literal::Word(s) => s.hash(state),
            Literal::Operator(op) => op.hash(state),
            Literal::Pointer(p) => p.as_raw_ptr().hash(state),
        }
    }
}
