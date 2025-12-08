pub trait Rule {
    type Item: ?Sized;
    type Result<'a> where Self: 'a;

    fn test<'a>(&'a self, eval: &'a Self::Item) -> Self::Result<'a>;
}

pub trait RuleSet: Default {
    type Item: ?Sized;
    type Result<'a> where Self: 'a;
    type Rule: Rule<Item = Self::Item>;

    fn get_rules(&self) -> &Vec<Self::Rule>;
    fn insert(&mut self, rule: Self::Rule);

    fn test_all<'a>(&'a self, obj: &'a Self::Item) -> Vec<<Self::Rule as Rule>::Result<'a>> {
        self.get_rules()
            .iter()
            .map(|rule| rule.test(obj))
            .collect()
    }

    fn first_match<'a>(&'a self, obj: &'a Self::Item) -> <Self::Rule as Rule>::Result<'a>
    where
        <Self::Rule as Rule>::Result<'a>: Default,
    {
        self.get_rules()
            .iter()
            .map(|rule| rule.test(obj))
            .find(|_| true)
            .unwrap_or_default()
    }
}
