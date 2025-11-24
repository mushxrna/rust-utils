use crate::parser::*;

pub struct StringBuffer {
    source: String,
}

impl StringBuffer {
    pub fn new(source: Option<String>) -> StringBuffer {
        Self {
            source: match source {
                Some(string) => string,
                None => String::new(),
            },
        }
    }

    pub fn push_str(&mut self, str: &str) {
        self.source.push_str(str);
    }

    pub fn push_string(&mut self, string: String) {
        self.source.extend(string.chars());
    }

    pub fn push_chars(&mut self, chars: Vec<char>) {
        self.source.extend(chars.iter());
    }

    pub fn push_char(&mut self, char: char) {
        self.source.push(char);
    }

    pub fn clear(&mut self) {
        self.source.clear();
    }

    pub fn ref_str(&self) -> &str {
        self.source.as_str()
    }

    pub fn ref_string(&self) -> &String {
        &self.source
    }

    pub fn ref_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.source.chars()
    }

    pub fn pull_string(&mut self) -> String {
        let string = self.source.clone();
        self.clear();
        string
    }

    pub fn pull_chars(&mut self) -> Vec<char> {
        let chars = self.source.clone().chars().collect();
        self.source.clear();
        chars
    }

    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    pub fn pull_literal(&mut self, ops: &OpTable, refs: &RefTable) -> Option<Literal> {
        if !self.is_empty() {
            let string = self.pull_string();
            let s_as_literal = Literal::Word(string.clone());

            if ops.contains(&string) {
                Some(Literal::Operator(
                    ops.operand_table.borrow()[&string].clone(),
                ))
            } else if refs.contains(&s_as_literal) {
                Some(Literal::Expression(vec![
                    refs.retrieve(&s_as_literal).clone(),
                ]))
            } else {
                Some(Literal::Word(string))
            }
        } else {
            None
        }
    }
}
