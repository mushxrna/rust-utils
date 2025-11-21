use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::generics::Byteable;
use crate::parser::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WordKindId(pub String);

pub trait KindWrapper<T>: Iop {}

pub trait Iop: Byteable + Clone {
    fn from_literal(literal: Literal) -> Result<Box<dyn Any>, String>
    where
        Self: Sized;

    fn to_literal(&self) -> Literal;

    fn id() -> WordKindId
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
    fn as_byteable(&self) -> &dyn Byteable;

    fn parse_rule() -> fn(String) -> Option<WordKindId>
    where
        Self: Sized;
}

pub struct TypeTable {
    id_to_parser: HashMap<WordKindId, Box<dyn Fn(Literal) -> Result<Box<dyn Any>, String>>>,
    id_to_byteable: HashMap<WordKindId, Box<dyn Fn(Literal) -> Box<dyn Byteable>>>,
    id_to_serializer: HashMap<WordKindId, Box<dyn Fn(&dyn Any) -> Literal>>,
    typeid_to_id: HashMap<TypeId, WordKindId>,
    parsing_rules: Vec<fn(String) -> Option<WordKindId>>,
}

impl TypeTable {
    pub fn new() -> Self {
        Self {
            id_to_parser: HashMap::new(),
            id_to_serializer: HashMap::new(),
            id_to_byteable: HashMap::new(),
            typeid_to_id: HashMap::new(),
            parsing_rules: vec![],
        }
    }

    pub fn register<Z: 'static, T: KindWrapper<Z> + 'static>(&mut self) {
        let id = T::id();
        let typeid = TypeId::of::<T>();

        self.id_to_parser
            .insert(id.clone(), Box::new(|l| T::from_literal(l)));

        self.id_to_serializer.insert(
            id.clone(),
            Box::new(|a| a.downcast_ref::<T>().map(|x| x.to_literal()).unwrap()),
        );

        self.id_to_byteable.insert(
            id.clone(),
            Box::new(|a| {
                match a.clone() {
                    Literal::TypedWord(s, id) => {
                        return Box::new(
                            T::from_literal(a)
                                .unwrap()
                                .downcast_ref::<T>()
                                .unwrap()
                                .clone(),
                        );
                    }
                    _ => panic!(),
                };
            }),
        );

        self.typeid_to_id.insert(typeid, id.clone());
        self.parsing_rules.push(T::parse_rule());
    }

    pub fn parse_into_typed<T: Iop + Clone + Sized + Any>(
        &self,
        l: &Literal,
        kind: &WordKindId,
    ) -> Result<T, String> {
        match l {
            Literal::Word(s) | Literal::TypedWord(s, _) => Ok(self.id_to_parser[&kind](l.clone())
                .unwrap()
                .downcast_ref::<T>()
                .unwrap()
                .clone()),
            //Literal::TypedWord(s, id) => Ok(),
            _ => Err("Err".to_owned()),
        }
    }

    pub fn serialize_into_literal(&self, value: &dyn Any) -> Result<Literal, String> {
        let typeid = value.type_id();
        let id = self.typeid_to_id[&typeid].clone();

        Ok(self.id_to_serializer[&id](value))
    }

    pub fn match_first_type(&self, lit: &Literal) -> Result<Literal, String> {
        let s = lit.as_string();
        for i in &self.parsing_rules {
            if let Some(id) = i(s.clone()) {
                return Ok(Literal::TypedWord(s, id));
            }
        }
        Err("No matching type pattern".to_owned())
    }

    pub fn auto_parse(&self, l: &Literal) -> Result<Box<dyn Byteable>, String> {
        let typed = self.match_first_type(l)?;
        match typed.clone() {
            Literal::TypedWord(s, id) => Ok(self.id_to_byteable[&id](typed)),
            _ => Err("Failed to determine type of ".to_owned() + &typed.as_string()),
        }
    }
}
