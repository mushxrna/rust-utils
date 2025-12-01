use std::any::Any;
use std::borrow::Borrow;

pub trait Rule {
    type Item: ?Sized;
    type Result;

    fn test<R: Borrow<Self::Item>>(&self, eval: &R) -> Self::Result;
}

pub trait RuleSet {
    type Item: ?Sized;
    type Result;

    type Rule: Rule<Item = Self::Item, Result = Self::Result>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all<R: Borrow<Self::Item>>(&self, obj: R) -> Vec<Self::Result> {
        self.get_rules()
            .iter()
            .map(|rule| -> Self::Result { rule.test(&obj) })
            .collect()
    }
}
