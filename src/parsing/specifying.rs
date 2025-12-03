use crate::parsing::rules::*;
use std::{borrow::Borrow, fmt::Result};

pub struct Specifier<T: RuleSet> {
    rule_set: T,
}

impl<T: RuleSet> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify_sliced<I, R: Borrow<T::Item>>(&self, slice: &[R]) -> Vec<I>
    where
        T: RuleSet<Result = Option<I>>,
    {
        let mut result = vec![];

        for i in slice.iter() {
            let mut found = false;
            for x in self.rule_set.get_rules() {
                if let Some(val) = x.test(i) {
                    result.push(val);
                    found = true;
                    break;
                }
            }
            (!found).then(|| panic!("could not specify molecule!"));
        }

        result
    }

    pub fn specify_ref<I>(&self, item: &T::Item) -> I
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
