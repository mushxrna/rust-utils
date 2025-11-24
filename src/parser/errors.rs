use std::fmt::{Display, Formatter, Result};

use crate::parser::{Literal, WordKindId};

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

#[derive(Debug)]
pub enum InternalError {
    CannotReferenceExprStr,
    CannotReferencePtrStr,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InternalError::CannotReferenceExprStr | InternalError::CannotReferencePtrStr => {
                let s = "Cannot retrieve reference for reference or pointer strings.";
                write!(f, "{}", s)
            }
        }
    }
}

impl std::error::Error for InternalError {}

#[derive(Debug)]
pub enum TypeTableError {
    CannotParseInto(Literal, WordKindId),
    CannotMatchType(Literal),
}

impl Display for TypeTableError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::CannotParseInto(l, id) => {
                let lit_str = &l.as_string();
                let id_str = &id.0;
                write!(
                    f,
                    "Cannot parse literal: ({}) into type: ({})",
                    lit_str, id_str
                )
            }
            Self::CannotMatchType(l) => {
                let lit_str = &l.as_string();
                write!(f, "Cannot identify type of literal: ({})", lit_str)
            }
        }
    }
}

impl std::error::Error for TypeTableError {}
