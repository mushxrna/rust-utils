pub trait Rule<'a> {
    type Item: ?Sized + 'a;
    type Result;

    fn test(&self, eval: &'a Self::Item) -> Self::Result;
}

pub trait RuleSet<'a>: Default {
    type Item: ?Sized + 'a;
    type Result;
    type Rule: Rule<'a, Item = Self::Item, Result = Self::Result>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all(&self, obj: &'a Self::Item) -> Vec<Self::Result> {
        self.get_rules()
            .iter()
            .map(|rule| rule.test(obj))
            .collect()
    }

    fn first_match(&self, obj: &'a Self::Item) -> Self::Result
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
