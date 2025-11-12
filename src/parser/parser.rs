use std::collections::HashMap;
use std::rc::Rc;

use crate::extensions::string_ext::StringExt;
use crate::parser::{Literal, OpTable, Operand, StringBuffer};

pub struct ParseTreeBuilder {
    source: StringBuffer,
    ops: Rc<OpTable>,
}

impl ParseTreeBuilder {
    pub fn new(filepath: &str, ops: Rc<OpTable>) -> ParseTreeBuilder {
        let string = std::fs::read_to_string(filepath).unwrap();
        let source = Self::pre_process_string(&string);
        ParseTreeBuilder { source, ops }
    }

    pub fn new_from_str(source: &str, ops: Rc<OpTable>) -> ParseTreeBuilder {
        let source = Self::pre_process_string(source);
        ParseTreeBuilder { source, ops }
    }

    pub fn pre_process_string(str: &str) -> StringBuffer {
        StringBuffer::new(Some(str.replace("\n", " ") + " "))
    }

    pub fn expressionize_buffer(&self, buf: &StringBuffer) -> Literal {
        let chars: Vec<char> = buf.ref_chars().collect();

        let mut literals = Vec::new();
        let mut working_buffer = StringBuffer::new(None);

        let mut index: usize = 0;

        while index < chars.len() {
            match chars[index] {
                '(' => {
                    if let Some(l) = working_buffer.pull_literal(&self.ops) {
                        literals.push(l)
                    }

                    let close = buf
                        .ref_string()
                        .find_next_delimiter_index(index, ('(', ')'));

                    let new_buffer =
                        StringBuffer::new(Some(chars[(index + 1)..close].into_iter().collect()));

                    literals.push(Self::expressionize_buffer(self, &new_buffer));
                    index = close;
                }
                ' ' => {
                    if let Some(l) = working_buffer.pull_literal(&self.ops) {
                        literals.push(l)
                    }
                }
                any => working_buffer.push_char(any),
            }
            index = index + 1;
        }

        if let Some(l) = working_buffer.pull_literal(&self.ops) {
            literals.push(l)
        }

        Literal::Expression(literals)
    }

    pub fn build(self) -> ParseTree {
        let exp = self.expressionize_buffer(&self.source);
        ParseTree::new(exp)
    }
}

pub struct ParseTree {
    pub expression: Literal,
}

impl ParseTree {
    pub fn new(expression: Literal) -> ParseTree {
        ParseTree {
            expression: expression,
        }
    }
}
