// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use regex;

use Predicate;

/// An error that occurred during parsing or compiling a regular expression.
pub type RegexError = regex::Error;

/// Predicate that uses regex matching
///
/// This is created by the `predicate::str::is_match`.
#[derive(Clone, Debug)]
pub struct RegexPredicate {
    re: regex::Regex,
}

impl Predicate for RegexPredicate {
    type Item = str;

    fn eval(&self, variable: &str) -> bool {
        self.re.is_match(variable)
    }
}

/// Creates a new `Predicate` that uses a regular expression to match the string.
///
/// # Examples
///
/// ```
/// use predicates::predicate::*;
///
/// let predicate_fn = str::is_match("^Hel.o.*$").unwrap();
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Food World"));
/// ```
pub fn is_match<S>(pattern: S) -> Result<RegexPredicate, RegexError>
where
    S: AsRef<str>,
{
    regex::Regex::new(pattern.as_ref()).map(|re| RegexPredicate { re })
}
