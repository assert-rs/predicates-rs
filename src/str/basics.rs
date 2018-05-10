// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Predicate;

/// Predicate that checks for empty strings.
///
/// This is created by `predicates::str::is_empty`.
#[derive(Copy, Clone, Debug)]
pub struct IsEmptyPredicate {}

impl Predicate<str> for IsEmptyPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.is_empty()
    }
}

/// Creates a new `Predicate` that ensures a str is empty
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::is_empty();
/// assert_eq!(true, predicate_fn.eval(""));
/// assert_eq!(false, predicate_fn.eval("Food World"));
/// ```
pub fn is_empty() -> IsEmptyPredicate {
    IsEmptyPredicate {}
}

#[derive(Copy, Clone, Debug)]
enum PatternOp {
    StartsWith,
    EndsWith,
    Contains,
}

/// Predicate that checks for patterns.
///
/// This is created by `predicates::str::{starts_with, ends_with, contains}`.
#[derive(Clone, Debug)]
pub struct PatternPredicate {
    pattern: String,
    op: PatternOp,
}

impl Predicate<str> for PatternPredicate {
    fn eval(&self, variable: &str) -> bool {
        match self.op {
            PatternOp::StartsWith => variable.starts_with(&self.pattern),
            PatternOp::EndsWith => variable.ends_with(&self.pattern),
            PatternOp::Contains => variable.contains(&self.pattern),
        }
    }
}

/// Creates a new `Predicate` that ensures a str starts with `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::starts_with("Hello");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Goodbye World"));
/// ```
pub fn starts_with<P>(pattern: P) -> PatternPredicate
where
    P: Into<String>,
{
    PatternPredicate {
        pattern: pattern.into(),
        op: PatternOp::StartsWith,
    }
}

/// Creates a new `Predicate` that ensures a str ends with `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::ends_with("World");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Hello Moon"));
/// ```
pub fn ends_with<P>(pattern: P) -> PatternPredicate
where
    P: Into<String>,
{
    PatternPredicate {
        pattern: pattern.into(),
        op: PatternOp::EndsWith,
    }
}

/// Creates a new `Predicate` that ensures a str contains `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::contains("Two");
/// assert_eq!(true, predicate_fn.eval("One Two Three"));
/// assert_eq!(false, predicate_fn.eval("Four Five Six"));
/// ```
pub fn contains<P>(pattern: P) -> PatternPredicate
where
    P: Into<String>,
{
    PatternPredicate {
        pattern: pattern.into(),
        op: PatternOp::Contains,
    }
}
