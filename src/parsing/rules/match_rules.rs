use crate::parsing::rules::{Rule, RuleSet};

//
// STRUCTS
//
pub enum MatchRuleResult<A> {
    Match(A),
    NoMatch,
}

//
// IMPL MATCHRULERESULT
//
impl<A> MatchRuleResult<A> {
    pub fn is_match(&self) -> bool {
        matches!(self, MatchRuleResult::Match(_))
    }

    pub fn as_ref(&self) -> Option<&A> {
        match self {
            MatchRuleResult::Match(a) => Some(a),
            MatchRuleResult::NoMatch => None,
        }
    }

    pub fn into_inner(self) -> Option<A> {
        match self {
            MatchRuleResult::Match(a) => Some(a),
            MatchRuleResult::NoMatch => None,
        }
    }

    pub fn map<B, F>(self, f: F) -> MatchRuleResult<B>
    where
        F: FnOnce(A) -> B,
    {
        match self {
            MatchRuleResult::Match(a) => MatchRuleResult::Match(f(a)),
            MatchRuleResult::NoMatch => MatchRuleResult::NoMatch,
        }
    }
}

impl<A> From<MatchRuleResult<A>> for Option<A> {
    fn from(result: MatchRuleResult<A>) -> Self {
        result.into_inner()
    }
}

impl<A> From<Option<A>> for MatchRuleResult<A> {
    fn from(opt: Option<A>) -> Self {
        match opt {
            Some(a) => MatchRuleResult::Match(a),
            None => MatchRuleResult::NoMatch,
        }
    }
}

impl<A> Default for MatchRuleResult<A> {
    fn default() -> Self {
        MatchRuleResult::NoMatch
    }
}

pub struct MatchRule<Item: ?Sized, Result, F>
where
    F: Fn(&Item) -> MatchRuleResult<Result>,
{
    pub rule: F,
    pub priority: usize,
    _marker: std::marker::PhantomData<fn(&Item) -> Result>,
}

pub struct MatchRuleSet<Item: ?Sized, Result, F>
where
    F: Fn(&Item) -> MatchRuleResult<Result>,
{
    pub match_rules: Vec<MatchRule<Item, Result, F>>,
}

//
// IMPL RULE
//
impl<A: ?Sized, B, F> Rule for MatchRule<A, B, F>
where
    F: Fn(&A) -> MatchRuleResult<B>,
{
    type Item = A;
    type Result<'a> = MatchRuleResult<B> where Self: 'a;

    fn test<'a>(&'a self, eval: &'a A) -> MatchRuleResult<B> {
        (self.rule)(eval)
    }
}

//
// IMPL RULESET
//
impl<A: ?Sized, B, F> RuleSet for MatchRuleSet<A, B, F>
where
    F: Fn(&A) -> MatchRuleResult<B>,
{
    type Item = A;
    type Result<'a> = MatchRuleResult<B> where Self: 'a;
    type Rule = MatchRule<A, B, F>;

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
impl<A: ?Sized, B, F> Default for MatchRuleSet<A, B, F>
where
    F: Fn(&A) -> MatchRuleResult<B>,
{
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}

//
// IMPL METHODS
//
impl<A: ?Sized, B, F> MatchRule<A, B, F>
where
    F: Fn(&A) -> MatchRuleResult<B>,
{
    pub fn new(rule: F, priority: usize) -> MatchRule<A, B, F> {
        MatchRule {
            rule,
            priority,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: ?Sized, B, F> MatchRuleSet<A, B, F>
where
    F: Fn(&A) -> MatchRuleResult<B>,
{
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
