use crate::parsing::rules::*;

pub struct Specifier<T> {
    rule_set: T,
}

impl<T> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify<'a, I>(&self, item: &'a T::Item) -> Option<I>
    where
        T: RuleSet<'a, Result = MatchRuleResult<I>>,
    {
        self.rule_set
            .test_all(item)
            .into_iter()
            .find(|r| r.is_match())
            .and_then(|r| r.into_inner())
    }
}
