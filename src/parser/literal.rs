use crate::parser::{BytePointer, errors::InternalError, heap::BytePtr};

#[derive(Debug)]
pub enum Literal {
    Word(String),
    Operator(Operand),
    Expression(Vec<Literal>),
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
    pub fn as_string(&self) -> &String {
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
            Operand::DropIn(s) => Operand::DropIn(self.as_string().clone()),
            Operand::Binary(s) => Operand::Binary(self.as_string().clone()),
            Operand::Function(s) => Operand::Function(self.as_string().clone()),
            Operand::Assignment(s) => Operand::Assignment(self.as_string().clone()),
        }
    }
}

impl Literal {
    pub fn as_string(&self) -> String {
        match self {
            Literal::Word(string) => string.clone(),
            Literal::Operator(op) => op.as_string().clone(),
            Literal::Expression(v) => v.iter().map(|x| x.as_string() + " ").collect::<String>(),
            Literal::Pointer(p) => p.as_raw_ptr().to_string(),
        }
    }

    pub fn ref_string(&self) -> Result<&String, InternalError> {
        match self {
            Literal::Word(string) => Ok(string),
            Literal::Operator(op) => Ok(op.as_string()),
            Literal::Expression(v) => Err(InternalError::CannotReference),
            Literal::Pointer(p) => Err(InternalError::CannotReference),
        }
    }
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        println!("Cloning Literal.");
        match self {
            Literal::Word(s) => Literal::Word(s.clone()),
            Literal::Operator(o) => Literal::Operator(o.clone()),
            Literal::Expression(v) => Literal::Expression(v.clone()),
            Literal::Pointer(p) => {
                let x = p.as_raw_ptr();
                let y: BytePointer<i32> = BytePointer::from_raw_ptr(x);
                Literal::Pointer(Box::new(y))
            }
        }
    }
}
