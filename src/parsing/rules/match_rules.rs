use crate::parsing::rules::{Rule, RuleSet, match_rules};
use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;

//
// STRUCTS
//
pub struct MatchRule<Item: ?Sized + Deref<Target: Sized>, Result> {
    pub rule: Box<dyn Fn(&Item::Target) -> Option<Result>>,
    pub priority: usize,
}

pub struct MatchRuleSet<Item: ?Sized + Deref<Target: Sized>, Result> {
    pub match_rules: Vec<MatchRule<Item, Result>>,
}
//
// IMPL RULE
//
impl<A: ?Sized + Deref<Target: Sized>, B> Rule for MatchRule<A, B> {
    type Item = A;
    type Result = Option<B>;

    fn test(&self, eval: &A::Target) -> Self::Result {
        (self.rule)(eval.deref())
    }
}
//
// IMPL RULESET
//
impl<A: ?Sized + Deref<Target: Sized>, B> RuleSet for MatchRuleSet<A, B> {
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
impl<A: ?Sized + Deref<Target: Sized>, B> Default for MatchRuleSet<A, B> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}
//
// IMPL METHODS
//
impl<A: ?Sized + Deref<Target: Sized>, B> MatchRule<A, B> {
    pub fn new(rule: Box<dyn Fn(&A::Target) -> Option<B>>, priority: usize) -> MatchRule<A, B> {
        MatchRule { rule, priority }
    }
}

impl<A: ?Sized + Deref<Target: Sized>, B> MatchRuleSet<A, B> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
