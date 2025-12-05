use crate::parsing::rules::Rule;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub struct ArgRule<Item, Result> {
    rule: Box<dyn Fn(&[Item]) -> Result>,
}

pub struct RuleMap<B, R> {
    map: HashMap<B, ArgRule<B, R>>,
}

impl<A: std::cmp::Eq + Hash, R> RuleMap<A, R> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: A, rule: Box<dyn Fn(&[A]) -> R>) {
        self.map.insert(item, ArgRule { rule });
    }

    pub fn evaluate(&self, item: A, args: &[A]) -> Option<R> {
        Some((self.map.get(&item)?.rule)(args))
    }
}
