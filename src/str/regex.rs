// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use regex;

use reflection;
use Predicate;

/// An error that occurred during parsing or compiling a regular expression.
pub type RegexError = regex::Error;

/// Predicate that uses regex matching
///
/// This is created by the `predicate::str::is_match`.
#[derive(Debug, Clone)]
pub struct RegexPredicate {
    re: regex::Regex,
}

impl RegexPredicate {
    /// Require a specific count of matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::is_match("T[a-z]*").unwrap().count(3);
    /// assert_eq!(true, predicate_fn.eval("One Two Three Two One"));
    /// assert_eq!(false, predicate_fn.eval("One Two Three"));
    /// ```
    pub fn count(self, count: usize) -> RegexMatchesPredicate {
        RegexMatchesPredicate { re: self.re, count }
    }
}

impl Predicate<str> for RegexPredicate {
    fn eval(&self, variable: &str) -> bool {
        self.re.is_match(variable)
    }
}

impl reflection::PredicateReflection for RegexPredicate {}

impl fmt::Display for RegexPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.is_match({})", self.re)
    }
}

/// Predicate that checks for repeated patterns.
///
/// This is created by `predicates::str::is_match(...).count`.
#[derive(Debug, Clone)]
pub struct RegexMatchesPredicate {
    re: regex::Regex,
    count: usize,
}

impl Predicate<str> for RegexMatchesPredicate {
    fn eval(&self, variable: &str) -> bool {
        self.re.find_iter(variable).count() == self.count
    }
}

impl reflection::PredicateReflection for RegexMatchesPredicate {}

impl fmt::Display for RegexMatchesPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.is_match({}).count({})", self.re, self.count)
    }
}

/// Creates a new `Predicate` that uses a regular expression to match the string.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::is_match("^Hel.o.*$").unwrap();
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Food World"));
/// ```
pub fn is_match<S>(pattern: S) -> Result<RegexPredicate, RegexError>
where
    S: AsRef<str>,
{
    regex::Regex::new(pattern.as_ref()).map(|re| RegexPredicate { re })
}
