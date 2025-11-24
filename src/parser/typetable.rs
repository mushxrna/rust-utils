use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use crate::generics::Byteable;
use crate::parser::errors::*;
use crate::parser::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WordKindId(pub String);

pub trait KindWrapper<T>: Iop {}

pub trait Iop: Byteable + Clone {
    fn from_literal(literal: &Literal) -> Result<Box<dyn Any>, String>
    where
        Self: Sized;

    fn to_literal(&self) -> Literal;

    fn id() -> WordKindId
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
    fn as_byteable(&self) -> &dyn Byteable;
    fn clone_boxed(&self) -> Box<Self>;

    fn parse_rule() -> fn(&str) -> Option<WordKindId>
    where
        Self: Sized;
}

pub struct TypeTable {
    id_to_parser: HashMap<WordKindId, Box<dyn Fn(&Literal) -> Result<Box<dyn Any>, String>>>,
    id_to_byteable: HashMap<WordKindId, Box<dyn Fn(&Literal) -> Result<Box<dyn Byteable>, String>>>,
    id_to_serializer: HashMap<WordKindId, Box<dyn Fn(&dyn Any) -> Result<Literal, String>>>,
    typeid_to_id: HashMap<TypeId, WordKindId>,
    parsing_rules: Vec<fn(&str) -> Option<WordKindId>>,
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
            .insert(id.clone(), Box::new(|l| T::from_literal(&l)));

        self.id_to_serializer.insert(
            id.clone(),
            Box::new(|a| {
                if let Some(x) = a.downcast_ref::<T>() {
                    Ok(x.to_literal())
                } else {
                    Err("Error serializing into literal".to_string())
                }
            }),
        );

        self.id_to_byteable.insert(
            id.clone(),
            Box::new(|a| {
                if let Literal::TypedWord(s, id) = &a {
                    if let Some(x) = T::from_literal(&a)?.downcast_ref::<T>() {
                        Ok(x.clone_boxed())
                    } else {
                        Err("Could not get byteable: ".to_string() + &a.as_cow())
                    }
                } else {
                    Err("Cannot get byteable of untyped word.".to_string())
                }
            }),
        );

        self.typeid_to_id.insert(typeid, id.clone());
        self.parsing_rules.push(T::parse_rule());
    }

    pub fn parse_into_typed<T: KindWrapper<Z> + 'static, Z>(
        &self,
        l: &Literal,
    ) -> Result<T, String> {
        match l {
            Literal::Word(s) | Literal::TypedWord(s, _) => {
                let lit = l;
                let val = self.id_to_parser[&T::id()](lit)?;
                let typed = val.downcast_ref::<T>();

                if let Some(x) = typed {
                    Ok(x.clone())
                } else {
                    Err(TypeTableError::CannotParseInto(l.clone(), T::id()).to_string())
                }
            }
            _ => Err(TypeTableError::CannotParseInto(l.clone(), T::id()).to_string()),
        }
    }

    pub fn serialize_into_literal(&self, value: &dyn Any) -> Result<Literal, String> {
        let typeid = value.type_id();
        let id = self.typeid_to_id[&typeid].clone();

        Ok(self.id_to_serializer[&id](value)?)
    }

    pub fn match_first_type(&self, lit: &Literal) -> Result<Literal, String> {
        let s = lit.as_cow();
        for i in &self.parsing_rules {
            if let Some(id) = i(&s) {
                return Ok(Literal::TypedWord(s.to_string(), id));
            }
        }
        Err("No matching type pattern for literal: (".to_string() + &lit.as_cow() + ")")
    }

    pub fn auto_parse(&self, l: &Literal) -> Result<Box<dyn Byteable>, String> {
        let typed = self.match_first_type(l)?;
        match &typed {
            Literal::TypedWord(s, id) => Ok(self.id_to_byteable[&id](&typed)?),
            _ => Err("Failed to determine type of ".to_owned() + &typed.as_string()),
        }
    }
}
