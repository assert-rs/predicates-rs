// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow;
use std::fmt;

use crate::reflection;
use crate::Predicate;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SimilarOp {
    Similar,
    Different,
}

impl SimilarOp {
    fn eval(self, similar: bool) -> bool {
        match self {
            SimilarOp::Similar => similar,
            SimilarOp::Different => !similar,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SimilarLimit {
    Ratio(f32),
    Changes(u32),
    // TODO: DiffOps(u32,u32,u32) variant that counts insert/delete/replace separately
    //       It would also be nice to have the option to not generate DiffOp::Replace
}

impl fmt::Display for SimilarLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimilarLimit::Ratio(r) => write!(f, "ratio({})", r),
            SimilarLimit::Changes(d) => write!(f, "changes({})", d),
        }
    }
}

/// Predicate that diffs two strings.
///
/// This is created by [`similar3()`](crate::str::similar3) or [`diff3()`](crate::str::diff3).
#[derive(Debug, Clone, PartialEq)]
pub struct SimilarPredicate {
    old: borrow::Cow<'static, str>,
    algorithm: similar::Algorithm,
    op: SimilarOp,
    limit: SimilarLimit,
}

impl SimilarPredicate {
    /// The maximum allowed number of changes.
    ///
    /// Default: `0`
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use predicates::str::similar3;
    ///
    /// let s1 = "Hello World!";
    /// let s2 = "Hello... World?";
    ///
    /// assert_eq!(false, similar3(s1).eval(s2));
    /// assert_eq!(true, similar3(s1).changes(2).eval(s2));
    /// assert_eq!("changes(2)", &similar3(s1).changes(2).find_case(true, s2).unwrap().product_value("measure").unwrap());
    /// ```
    pub fn changes(mut self, changes: u32) -> Self {
        self.limit = SimilarLimit::Changes(changes);
        self
    }

    /// The minimum required [similarity](similar::get_diff_ratio).
    ///
    /// Default: `1.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use predicates::str::similar3;
    ///
    /// let s1 = "Hello World!";
    /// let s2 = "Hello... World?";
    ///
    /// assert_eq!(false, similar3(s1).eval(s2));
    /// assert_eq!(true, similar3(s1).ratio(0.8).eval(s2));
    /// assert_eq!("ratio(0.8148148)", &similar3(s1).find_case(false, s2).unwrap().product_value("measure").unwrap());
    /// ```
    pub fn ratio(mut self, ratio: f32) -> Self {
        // FIXME: clamp to 0..1 ? Panic ?
        self.limit = SimilarLimit::Ratio(ratio);
        self
    }

    /// The [diffing algorithm](similar::Algorithm).
    ///
    /// Default: [`Myers`](similar::Algorithm::Myers)
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use predicates::str::similar3;
    /// use similar::Algorithm::*;
    ///
    /// let s1 = "Hello World!";
    /// let s2 = "Hello... World?";
    /// for (pred, expect) in vec![(similar3(s1), "ratio(0.8148148)"),
    ///                            (similar3(s1).algorithm(Myers), "ratio(0.8148148)"),
    ///                            (similar3(s1).algorithm(Lcs), "ratio(0.44444445)"),
    ///                            (similar3(s1).algorithm(Patience), "ratio(0.8148148)")] {
    ///   let m = pred.find_case(false, s2).unwrap().product_value("measure").unwrap();
    ///   assert_eq!(expect, &m);
    /// }
    /// ```
    pub fn algorithm(mut self, alg: similar::Algorithm) -> Self {
        self.algorithm = alg;
        self
    }

    fn new(old: borrow::Cow<'static, str>, op: SimilarOp) -> Self {
        Self {
            old,
            algorithm: similar::Algorithm::Myers,
            op,
            limit: SimilarLimit::Ratio(1.0),
        }
    }

    fn run(&self, new: &str) -> (bool, Vec<similar::DiffOp>, SimilarLimit) {
        use similar::DiffOp::*;
        use SimilarLimit::*;

        let chunks =
            similar::capture_diff_slices(self.algorithm, self.old.as_bytes(), new.as_bytes());
        match self.limit {
            Ratio(r) => {
                let ratio = similar::get_diff_ratio(&chunks, self.old.len(), new.len());
                (self.op.eval(ratio >= r), chunks, Ratio(ratio))
            }
            Changes(d) => {
                let changes = chunks.iter().fold(0, |n, c| match c {
                    Equal { .. } => n,
                    _ => n + 1,
                });
                (self.op.eval(changes <= d), chunks, Changes(changes))
            }
        }
    }

    /// Output a formated diff
    fn diff(&self, old: &str, new: &str, chunks: &[similar::DiffOp]) -> String {
        use similar::DiffOp::*;
        use std::fmt::Write;

        let mut f = String::with_capacity(chunks.len());
        for c in chunks {
            match *c {
                Equal { old_index, len, .. } => write!(f, "{}", &old[old_index..old_index + len]),
                Delete {
                    old_index, old_len, ..
                } => write!(f, "\x1b[92m{}\x1b[0m", &old[old_index..old_index + old_len]),
                Insert {
                    new_index, new_len, ..
                } => write!(f, "\x1b[91m{}\x1b[0m", &new[new_index..new_index + new_len]),
                Replace {
                    new_index, new_len, ..
                } => write!(f, "\x1b[95m{}\x1b[0m", &new[new_index..new_index + new_len]),
            }
            .expect("write to String")
        }
        f
    }
}

impl Predicate<str> for SimilarPredicate {
    fn eval(&self, new: &str) -> bool {
        self.run(new).0
    }

    fn find_case<'a>(&'a self, expected: bool, new: &str) -> Option<reflection::Case<'a>> {
        let (result, chunks, measure) = self.run(new);
        if result == expected {
            Some(
                reflection::Case::new(Some(self), result)
                    .add_product(reflection::Product::new("measure", measure))
                    .add_product(reflection::Product::new(
                        "diff",
                        self.diff(&self.old, new, &chunks),
                    )),
            )
        } else {
            None
        }
    }
}

impl reflection::PredicateReflection for SimilarPredicate {
    fn parameters<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Parameter<'a>> + 'a> {
        let params = vec![reflection::Parameter::new("original", &self.old)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for SimilarPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.op {
            SimilarOp::Similar => write!(f, "original is similar to var by {}", self.limit),
            SimilarOp::Different => write!(f, "original differs from var by {}", self.limit)
        }
    }
}

/// Creates a new `Predicate` that diffs two strings.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
/// use predicates::str::diff3;
///
/// let predicate = diff3("Hello World");
/// assert_eq!(false, predicate.eval("Hello World"));
/// assert_eq!(true, predicate.eval("Goodbye World"));
///
/// let diff = predicate.find_case(true, "Goodbye World!").unwrap().product_value("diff").unwrap();
/// assert_eq!("\x1b[95mGo\x1b[0mo\x1b[91mdbye\x1b[0m World\x1b[91m!\x1b[0m", &diff);
/// ```
pub fn diff3<S>(old: S) -> SimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    SimilarPredicate::new(old.into(), SimilarOp::Different)
}

/// Creates a new `Predicate` that checks strings for how similar they are.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
/// use predicates::str::similar3;
///
/// let predicate = predicate::str::similar3("Hello World");
/// assert_eq!(true, predicate.eval("Hello World"));
/// assert_eq!(false, predicate.eval("Goodbye World"));
///
/// let diff = predicate.find_case(false, "Goodbye World!").unwrap().product_value("diff").unwrap();
/// assert_eq!("\x1b[95mGo\x1b[0mo\x1b[91mdbye\x1b[0m World\x1b[91m!\x1b[0m", &diff);
/// ```
pub fn similar3<S>(old: S) -> SimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    SimilarPredicate::new(old.into(), SimilarOp::Similar)
}
