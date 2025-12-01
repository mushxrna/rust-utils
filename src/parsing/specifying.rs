use crate::parsing::rules::*;
use std::borrow::Borrow;

pub struct Specifier<T: RuleSet> {
    rule_set: T,
}

impl<I, T: RuleSet<Result = Option<I>>> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify_sliced<R: Borrow<T::Item> + Borrow<dyn std::fmt::Display>>(
        &self,
        slice: &[R],
    ) -> Vec<I> {
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
            (!found).then(|| {
                panic!(
                    "could not specify molecule! molecule : {}",
                    <R as Borrow<dyn std::fmt::Display>>::borrow(i)
                )
            });
        }

        result
    }
}
