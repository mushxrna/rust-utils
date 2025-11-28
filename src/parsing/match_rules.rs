use std::any::Any;
use std::borrow::Borrow;
use crate::parsing::{Rule, RuleSet};

pub struct MatchRule<'a, T: ?Sized, Z> {
    pub rule: Box<&'a dyn Fn(&T) -> Option<Z>>,
    pub priority: usize
}

pub struct MatchRuleSet<'a, T: ?Sized, Z> {
    pub match_rules: Vec<MatchRule<'a, T, Z>>
}

impl<'a, T: ?Sized, Z> Rule for MatchRule<'a, T, Z> {
    type Item = &'a T;
    type Result = Option<Z>;

    fn test(&self, eval: &Self::Item) -> Self::Result {
        (self.rule)(eval)
    }
}

impl<'a, T: ?Sized, Z> RuleSet for MatchRuleSet<'a, T, Z> {
    type Item = &'a T;
    type Result = Option<Z>;

    type Rule = MatchRule<'a, T, Z>;

    fn get_rules(&self) -> &Vec<Self::Rule> {
        &self.match_rules
    }

    fn insert(&mut self, rule: Self::Rule) {
        self.match_rules.push(rule);
        self.priority_sort();
    }

}

impl<'a, T: ?Sized, Z> MatchRule<'a, T, Z> {
    pub fn new(rule: Box<&'a dyn Fn(&T) -> Option<Z>>, priority: usize) -> MatchRule<'a, T, Z> {
        MatchRule {
            rule,
            priority
        }
    }
}

impl<'a, T: ?Sized, Z> MatchRuleSet<'a, T, Z> {
    fn priority_sort(&mut self) {
        self.match_rules.sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}