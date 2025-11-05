use std::fmt::{Display, Formatter, Result};

use crate::parser::Literal;

#[derive(Debug)]
pub enum ParseError {
    LiteralConversion(Literal, Literal),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ParseError::LiteralConversion(from, to) => {
                write!(
                    f,
                    "Cannot convert from {} to {}",
                    from.as_string(),
                    to.as_string()
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}
