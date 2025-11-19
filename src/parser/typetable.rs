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

    fn id() -> WordKindId
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;

    fn parse_rule() -> &'static dyn Fn(String) -> Option<WordKindId>
    where
        Self: Sized;
}

pub struct TypeTable {
    id_to_parser: HashMap<WordKindId, Box<dyn Fn(Literal) -> Result<Box<dyn Any>, String>>>,
    id_to_serializer: HashMap<WordKindId, Box<dyn Fn(&dyn Any) -> Literal>>,
    typeid_to_id: HashMap<TypeId, WordKindId>,
    parsing_rules: Vec<&'static dyn Fn(String) -> Option<WordKindId>>,
}

impl TypeTable {
    pub fn new() -> Self {
        Self {
            id_to_parser: HashMap::new(),
            id_to_serializer: HashMap::new(),
            typeid_to_id: HashMap::new(),
            parsing_rules: vec![],
        }
    }

    pub fn register<T: Iop>(&mut self) {
        let id = T::id();
        let typeid = TypeId::of::<T>();

        self.id_to_parser
            .insert(id.clone(), Box::new(|l| T::from_literal(l)));

        self.id_to_serializer.insert(
            id.clone(),
            Box::new(|a| a.downcast_ref::<T>().map(|x| x.to_literal()).unwrap()),
        );
        self.typeid_to_id.insert(typeid, id.clone());
        self.parsing_rules.push(T::parse_rule());
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

    pub fn serialize_into_literal(&self, value: &dyn Any) -> Result<Literal, String> {
        let typeid = value.type_id();
        let id = self.typeid_to_id[&typeid].clone();

        Ok(self.id_to_serializer[&id](value))
    }
}
