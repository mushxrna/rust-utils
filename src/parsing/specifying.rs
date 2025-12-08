use crate::parsing::rules::*;

pub struct Specifier<T: RuleSet> {
    rule_set: T,
}

impl<T: RuleSet> Specifier<T> {
    pub fn new(ruleset: T) -> Specifier<T> {
        Specifier { rule_set: ruleset }
    }

    pub fn specify<I>(&self, item: &T::Item) -> Option<I>
    where
        T: RuleSet<Result = MatchRuleResult<I>>,
    {
        self.rule_set
            .test_all(item)
            .into_iter()
            .find(|r| r.is_match())
            .and_then(|r| r.into_inner())
    }
}
