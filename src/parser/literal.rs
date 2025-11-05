#[derive(Debug, Clone)]
pub enum Literal {
    Word(String),
    Operator(Operand),
    Expression(Vec<Literal>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Operand {
    Binary(String),
}

impl Operand {
    pub fn as_string(&self) -> String {
        match self {
            Operand::Binary(op) => op.clone(),
            _ => (String::from("x")),
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
            Literal::Operator(op) => op.clone().as_string(),
            Literal::Expression(v) => v.iter().map(|x| x.as_string()).collect::<String>(),
        }
    }
}
