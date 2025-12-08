use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;

pub trait Rule {
    type Item: ?Sized + Deref<Target: Sized>;
    type Result;

    fn test(&self, eval: &<Self::Item as Deref>::Target) -> Self::Result;
}

pub trait RuleSet: Default {
    type Item: ?Sized + Deref<Target: Sized>;
    type Result;

    type Rule: Rule<Item = Self::Item, Result = Self::Result>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all(&self, obj: &Self::Item) -> Vec<Self::Result> {
        self.get_rules()
            .iter()
            .map(|rule| -> Self::Result { rule.test(&obj) })
            .collect()
    }
}
