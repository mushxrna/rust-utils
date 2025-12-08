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

pub struct MatchRule<Item: ?Sized, Result> {
    pub rule: for<'a> fn(&'a Item) -> MatchRuleResult<Result>,
    pub priority: usize,
}

pub struct MatchRuleSet<Item: ?Sized, Result> {
    pub match_rules: Vec<MatchRule<Item, Result>>,
}
//
// IMPL RULE
//
impl<A: ?Sized, B> Rule for MatchRule<A, B> {
    type Item = A;
    type Result = MatchRuleResult<B>;

    fn test(&self, eval: &A) -> MatchRuleResult<B> {
        (self.rule)(eval)
    }
}
//
// IMPL RULESET
//
impl<A: ?Sized, B> RuleSet for MatchRuleSet<A, B> {
    type Item = A;
    type Result = MatchRuleResult<B>;
    type Rule = MatchRule<A, B>;

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
impl<A: ?Sized, B> Default for MatchRuleSet<A, B> {
    fn default() -> Self {
        Self {
            match_rules: vec![],
        }
    }
}

impl<A> Default for MatchRuleResult<A> {
    fn default() -> Self {
        MatchRuleResult::NoMatch
    }
}
//
// IMPL METHODS
//
impl<A: ?Sized, B> MatchRule<A, B> {
    pub fn new(rule: for<'a> fn(&'a A) -> MatchRuleResult<B>, priority: usize) -> MatchRule<A, B> {
        MatchRule { rule, priority }
    }
}

impl<A: ?Sized, B> MatchRuleSet<A, B> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
