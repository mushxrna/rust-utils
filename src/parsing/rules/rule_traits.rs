pub trait Rule {
    type Item: ?Sized;
    type Result;

    fn test(&self, eval: &Self::Item) -> Self::Result;
}

pub trait RuleSet: Default {
    type Item: ?Sized;
    type Result;
    type Rule: Rule<Item = Self::Item, Result = Self::Result>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all(&self, obj: &Self::Item) -> Vec<Self::Result> {
        self.get_rules()
            .iter()
            .map(|rule| rule.test(obj))
            .collect()
    }

    fn first_match(&self, obj: &Self::Item) -> Self::Result
    where
        Self::Result: Default,
    {
        self.get_rules()
            .iter()
            .map(|rule| rule.test(obj))
            .find(|_| true)
            .unwrap_or_default()
    }
}
