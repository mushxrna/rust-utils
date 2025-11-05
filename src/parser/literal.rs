#[derive(Debug)]
pub enum Literal {
    Word(String),
    Operator(String),
    Expression(Vec<Literal>),
}

impl Literal {
    pub fn detail_string(&self) -> String {
        match self {
            Literal::Word(string) => String::new() + " (WORD: " + &string + " ) ",
            Literal::Operator(string) => String::new() + " (OP: " + &string + " ) ",
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
            Literal::Operator(string) => string.clone(),
            Literal::Expression(v) => v.iter().map(|x| x.as_string()).collect::<String>(),
        }
    }
}
