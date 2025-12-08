use crate::parsing::rules::{Rule, RuleSet, match_rules};
use std::any::Any;
use std::borrow::Borrow;

//
// STRUCTS
//
pub struct MatchRule<'a, Item: ?Sized, Result> {
    pub rule: Box<dyn Fn(&Item) -> Option<Result> + 'a>,
    pub priority: usize,
}

pub struct MatchRuleSet<'a, Item: ?Sized, Result> {
    pub match_rules: Vec<MatchRule<'a, Item, Result>>,
}
//
// IMPL RULE
//
impl<'a, A: ?Sized, B> Rule for MatchRule<'a, A, B> {
    type Item = A;
    type Result = Option<B>;

    fn test<F: Borrow<Self::Item>>(&self, eval: &F) -> Self::Result {
        (self.rule)(eval.borrow())
    }
}
//
// IMPL RULESET
//
impl<'a, A: ?Sized, B> RuleSet for MatchRuleSet<'a, A, B> {
    type Item = A;
    type Result = Option<B>;

    type Rule = MatchRule<'a, A, B>;

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
impl<'a, A: ?Sized, B> Default for MatchRuleSet<'a, A, B> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}
//
// IMPL METHODS
//
impl<'a, A: ?Sized, B> MatchRule<'a, A, B> {
    pub fn new(rule: Box<dyn Fn(&A) -> Option<B>>, priority: usize) -> MatchRule<'a, A, B> {
        MatchRule { rule, priority }
    }
}

impl<'a, A: ?Sized, B> MatchRuleSet<'a, A, B> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
