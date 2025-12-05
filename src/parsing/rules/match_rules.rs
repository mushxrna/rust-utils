use crate::parsing::rules::{Rule, RuleSet, match_rules};
use std::any::Any;
use std::borrow::Borrow;

//
// STRUCTS
//
pub struct MatchRule<Item: ?Sized, Result> {
    pub rule: Box<dyn Fn(&Item) -> Option<Result>>,
    pub priority: usize,
}

pub struct MatchRuleSet<Item: ?Sized, Result> {
    pub match_rules: Vec<MatchRule<Item, Result>>,
}
//
// IMPL RULE
//
impl<A: ?Sized, B> Rule for MatchRule<A, B> {
    type Item = A;
    type Result = Option<B>;

    fn test<F: Borrow<Self::Item>>(&self, eval: &F) -> Self::Result {
        (self.rule)(eval.borrow())
    }
}
//
// IMPL RULESET
//
impl<A: ?Sized, B> RuleSet for MatchRuleSet<A, B> {
    type Item = A;
    type Result = Option<B>;

    type Rule = MatchRule<A, B>;

    fn get_rules(&self) -> &Vec<Self::Rule> {
        &self.match_rules
    }

    fn insert(&mut self, rule: Self::Rule) {
        self.match_rules.push(rule);
        self.priority_sort();
    }
}
//
// IMPL DEFAULT
//
impl<A: ?Sized, B> Default for MatchRuleSet<A, B> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}
//
// IMPL METHODS
//
impl<A: ?Sized, B> MatchRule<A, B> {
    pub fn new(rule: Box<dyn Fn(&A) -> Option<B>>, priority: usize) -> MatchRule<A, B> {
        MatchRule { rule, priority }
    }
}

impl<A: ?Sized, B> MatchRuleSet<A, B> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
