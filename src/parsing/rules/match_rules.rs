use crate::parsing::rules::{Rule, RuleSet, TypeConstructor};
//
// STRUCTS
//
pub enum MatchRuleResult<A> {
    Match(A),
    NoMatch,
}

pub struct MatchRule<Item: ?Sized, TC: TypeConstructor> {
    pub rule: for<'a> fn(&'a Item) -> MatchRuleResult<TC::Of<'a>>,
    pub priority: usize,
}

pub struct MatchRuleSet<Item: ?Sized, TC: TypeConstructor> {
    pub match_rules: Vec<MatchRule<Item, TC>>,
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

//
// IMPL RULE
//
impl<A: ?Sized, R: TypeConstructor> Rule for MatchRule<A, R> {
    type Item = A;
    type Result<'a>
        = MatchRuleResult<R::Of<'a>>
    where
        Self: 'a;

    fn test<'a>(&'a self, eval: &'a A) -> MatchRuleResult<R::Of<'a>> {
        (self.rule)(eval)
    }
}

//
// IMPL RULESET
//
impl<A: ?Sized, R: TypeConstructor> RuleSet for MatchRuleSet<A, R> {
    type Item = A;
    type Result<'a>
        = MatchRuleResult<R::Of<'a>>
    where
        Self: 'a;
    type Rule = MatchRule<A, R>;

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
impl<A: ?Sized, R: TypeConstructor> Default for MatchRuleSet<A, R> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}

//
// IMPL METHODS
//
impl<A: ?Sized, R: TypeConstructor> MatchRule<A, R> {
    pub fn new(rule: for<'a> fn(&'a A) -> MatchRuleResult<R::Of<'a>>, priority: usize) -> Self {
        MatchRule { rule, priority }
    }
}

impl<A: ?Sized, R: TypeConstructor> MatchRuleSet<A, R> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
