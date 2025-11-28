use std::any::Any;
use std::borrow::Borrow;

pub trait Rule {
    type Item;
    type Result;
 
    fn test(&self, eval: &Self::Item) -> Self::Result;
}

pub trait RuleSet {
    type Item;
    type Result;

    type Rule: Rule<Item = Self::Item, Result=Self::Result>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all<R: Borrow<Self::Item>>(&self, obj: R) -> Vec<Self::Result> {
        self.get_rules().iter().map(|rule| -> Self::Result { rule.test(obj.borrow()) } ).collect()
    }
}