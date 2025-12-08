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

pub struct MatchRule<Item: ?Sized, F> {
    pub rule: F,
    pub priority: usize,
    _marker: std::marker::PhantomData<fn(&Item)>,
}

pub struct MatchRuleSet<Item: ?Sized, F> {
    pub match_rules: Vec<MatchRule<Item, F>>,
}

//
// IMPL RULE
//
impl<A: ?Sized, R, F> Rule for MatchRule<A, F>
where
    F: Fn(&A) -> R,
{
    type Item = A;
    type Result<'a> = R where Self: 'a;

    fn test<'a>(&'a self, eval: &'a A) -> R {
        (self.rule)(eval)
    }
}

//
// IMPL RULESET
//
impl<A: ?Sized, R, F> RuleSet for MatchRuleSet<A, F>
where
    F: Fn(&A) -> R,
{
    type Item = A;
    type Result<'a> = R where Self: 'a;
    type Rule = MatchRule<A, F>;

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
impl<A: ?Sized, F> Default for MatchRuleSet<A, F> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}

//
// IMPL METHODS
//
impl<A: ?Sized, F> MatchRule<A, F> {
    pub fn new(rule: F, priority: usize) -> MatchRule<A, F> {
        MatchRule {
            rule,
            priority,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: ?Sized, F> MatchRuleSet<A, F> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
