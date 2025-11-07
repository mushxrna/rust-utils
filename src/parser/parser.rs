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

    pub fn reduce_expression(&self, exp: Literal) -> Literal {
        if let Literal::Expression(v) = exp {
            let mut evaluated: Vec<Literal> = vec![];

            v.into_iter()
                .for_each(|f| evaluated.push(self.reduce_expression(f)));

            while evaluated.len() >= 1 {
                let mut found_operator = false;
                let mut i = 0;

                while i < evaluated.len() {
                    if let Literal::Operator(op) = &evaluated[i] {
                        match op {
                            Operand::Binary(s) => {
                                if i > 0 && i < evaluated.len() - 1 {
                                    let left = &evaluated[i - 1];
                                    let right = &evaluated[i + 1];

                                    let result = self.ops.call_by_operand(op, &vec![left, right]);

                                    evaluated.splice((i - 1)..=(i + 1), vec![result]);
                                    found_operator = true;
                                    break;
                                }
                            }

                            Operand::DropIn(s) => {
                                evaluated[i] = self.ops.call_by_operand(op, &vec![]);
                                found_operator = true;
                                break;
                            }

                            Operand::Function(s) => {
                                let mut args = vec![];
                                if let Literal::Expression(v) = &evaluated[i + 1] {
                                    args = v.iter().collect()
                                }

                                let result = self.ops.call_by_operand(op, &args);

                                if !args.is_empty() {
                                    evaluated.splice((i)..=(i + 1), vec![result]);
                                    found_operator = true;
                                    break;
                                }
                            }
                        }
                    }
                    i += 1;
                }

                if !found_operator {
                    break;
                }
            }

            if evaluated.len() == 1 {
                evaluated[0].clone()
            } else {
                Literal::Expression(evaluated)
            }
        } else {
            exp
        }
    }

    pub fn build(self) -> ParseTree {
        let exp = self.expressionize_buffer(&self.source);
        let eexp = self.reduce_expression(exp);
        ParseTree::new(eexp)
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
