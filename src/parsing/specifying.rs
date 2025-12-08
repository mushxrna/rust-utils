use crate::parsing::rules::*;

pub struct Specifier<T> {
    rule_set: T,
}

impl<T: RuleSet> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify<'a, I>(&'a self, item: &'a T::Item) -> Option<I>
    where
        <T::Rule as Rule>::Result<'a>: Into<Option<I>>,
    {
        self.rule_set
            .test_all(item)
            .into_iter()
            .find_map(|r| r.into())
    }
}
