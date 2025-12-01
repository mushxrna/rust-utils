use crate::parsing::rules::{Rule, RuleSet, match_rules};
use std::any::Any;
use std::borrow::Borrow;

pub struct MatchRule<T: ?Sized, Z> {
    pub rule: Box<dyn Fn(&T) -> Option<Z>>,
    pub priority: usize,
}

pub struct MatchRuleSet<T: ?Sized, Z> {
    pub match_rules: Vec<MatchRule<T, Z>>,
}

impl<T: ?Sized, Z> Default for MatchRuleSet<T, Z> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}

impl<T: ?Sized, Z> Rule for MatchRule<T, Z> {
    type Item = T;
    type Result = Option<Z>;

    fn test<F: Borrow<Self::Item>>(&self, eval: &F) -> Self::Result {
        (self.rule)(eval.borrow())
    }
}

impl<T: ?Sized, Z> RuleSet for MatchRuleSet<T, Z> {
    type Item = T;
    type Result = Option<Z>;

    type Rule = MatchRule<T, Z>;

    fn get_rules(&self) -> &Vec<Self::Rule> {
        &self.match_rules
    }

    fn insert(&mut self, rule: Self::Rule) {
        self.match_rules.push(rule);
        self.priority_sort();
    }
}

impl<T: ?Sized, Z> MatchRule<T, Z> {
    pub fn new(rule: Box<dyn Fn(&T) -> Option<Z>>, priority: usize) -> MatchRule<T, Z> {
        MatchRule { rule, priority }
    }
}

impl<T: ?Sized, Z> MatchRuleSet<T, Z> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
