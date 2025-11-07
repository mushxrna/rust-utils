#[derive(Debug)]
pub enum Literal {
    Word(String),
    Operator(Operand),
    Expression(Vec<Literal>),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Operand {
    Binary(String),
    DropIn(String),
    Function(String),
}

impl Operand {
    pub fn as_string(&self) -> &String {
        match self {
            Operand::Binary(op) => op,
            Operand::DropIn(op) => op,
            Operand::Function(op) => op,
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
        }
    }
}

impl Literal {
    pub fn detail_string(&self) -> String {
        match self {
            Literal::Word(string) => String::new() + " (WORD: " + &string + " ) ",
            Literal::Operator(op) => String::new() + " (OP: " + &op.as_string() + " ) ",
            Literal::Expression(literals) => {
                String::new()
                    + " (EXP: "
                    + literals
                        .iter()
                        .map(|literal| -> String { literal.detail_string() })
                        .collect::<String>()
                        .as_str()
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Literal::Word(string) => string.clone(),
            Literal::Operator(op) => op.as_string().clone(),
            Literal::Expression(v) => v.iter().map(|x| x.as_string() + " ").collect(),
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
        }
    }
}
