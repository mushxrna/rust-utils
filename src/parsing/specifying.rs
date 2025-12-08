use crate::parsing::rules::*;
use std::{borrow::Borrow, fmt::Result};

pub struct Specifier<T: RuleSet> {
    rule_set: T,
}

impl<T: RuleSet> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify_ref<'a, 'b, I>(&'a self, item: &'b T::Item) -> I
    where
        T: RuleSet<Result = Option<I>>,
    {
        self.rule_set
            .test_all(item)
            .into_iter()
            .find_map(|x| x)
            .unwrap()
    }
}
