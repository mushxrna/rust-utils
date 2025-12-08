use crate::parsing::rules::{Rule, RuleSet};

//
// TRAITS
//
pub trait MatchOutput<'a> {
    type Output;
}

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

pub struct MatchRule<Item: ?Sized, Result: for<'a> MatchOutput<'a>> {
    pub rule: for<'a> fn(&'a Item) -> MatchRuleResult<<Result as MatchOutput<'a>>::Output>,
    pub priority: usize,
}

pub struct MatchRuleSet<Item: ?Sized, Result: for<'a> MatchOutput<'a>> {
    pub match_rules: Vec<MatchRule<Item, Result>>,
}
//
// IMPL RULE
//
impl<'a, A: ?Sized + 'a, B: for<'b> MatchOutput<'b>> Rule<'a> for MatchRule<A, B> {
    type Item = A;
    type Result = MatchRuleResult<<B as MatchOutput<'a>>::Output>;

    fn test(&self, eval: &'a A) -> MatchRuleResult<<B as MatchOutput<'a>>::Output> {
        (self.rule)(eval)
    }
}
//
// IMPL RULESET
//
impl<'a, A: ?Sized + 'a, B: for<'b> MatchOutput<'b>> RuleSet<'a> for MatchRuleSet<A, B> {
    type Item = A;
    type Result = MatchRuleResult<<B as MatchOutput<'a>>::Output>;
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
impl<A: ?Sized, B: for<'a> MatchOutput<'a>> Default for MatchRuleSet<A, B> {
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
impl<A: ?Sized, B: for<'a> MatchOutput<'a>> MatchRule<A, B> {
    pub fn new(
        rule: for<'a> fn(&'a A) -> MatchRuleResult<<B as MatchOutput<'a>>::Output>,
        priority: usize,
    ) -> MatchRule<A, B> {
        MatchRule { rule, priority }
    }
}

impl<A: ?Sized, B: for<'a> MatchOutput<'a>> MatchRuleSet<A, B> {
    fn priority_sort(&mut self) {
        self.match_rules
            .sort_by_key(|rule| std::cmp::Reverse(rule.priority));
    }
}
