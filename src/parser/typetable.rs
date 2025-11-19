use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::parser::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WordKindId(pub String);

pub trait Iop: Any + Debug + Send + Sync {
    fn from_literal(literal: Literal) -> Result<Box<dyn Any>, String>
    where
        Self: Sized;

    fn to_literal(&self) -> Literal;

    fn id() -> &'static WordKindId
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
}

pub struct TypeTable {
    id_to_parser: HashMap<WordKindId, Box<dyn Fn(Literal) -> Result<Box<dyn Any>, String>>>,
    id_to_serializer: HashMap<WordKindId, Box<dyn Fn(&dyn Any) -> Literal>>,
}

impl TypeTable {
    pub fn new() -> Self {
        Self {
            id_to_parser: HashMap::new(),
            id_to_serializer: HashMap::new(),
        }
    }

    pub fn register<T: Iop>(&mut self) {
        let id = T::id();

        self.id_to_parser
            .insert(id.clone(), Box::new(|l| T::from_literal(l)));

        self.id_to_serializer.insert(
            id.clone(),
            Box::new(|a| a.downcast_ref::<T>().map(|x| x.to_literal()).unwrap()),
        );
    }

    pub fn parse_into_typed<T: Iop + Clone>(
        &self,
        l: &Literal,
        kind: WordKindId,
    ) -> Result<T, String> {
        match l {
            Literal::Word(s) => Ok(self.id_to_parser[&kind](l.clone())
                .unwrap()
                .downcast_ref::<T>()
                .unwrap()
                .clone()),
            _ => Err("Err".to_owned()),
        }
    }
}
