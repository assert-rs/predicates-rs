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
enum DistanceOp {
    Similar,
    Different,
}

impl DistanceOp {
    fn eval(self, limit: i32, distance: i32) -> bool {
        match self {
            DistanceOp::Similar => distance <= limit,
            DistanceOp::Different => limit < distance,
        }
    }
}

/// Predicate that diffs two strings.
///
/// This is created by the `predicate::str::similar`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DissimilarPredicate {
    orig: borrow::Cow<'static, str>,
    distance: i32,
    op: DistanceOp,
}

impl DissimilarPredicate {
    /// The maximum allowed edit distance.
    ///
    /// Default: `0`
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::similar2("Hello World!").distance(1);
    /// assert_eq!(true, predicate_fn.eval("Hello World!"));
    /// assert_eq!(true, predicate_fn.eval("Hello World"));
    /// assert_eq!(false, predicate_fn.eval("Hello World?"));
    /// ```
    pub fn distance(mut self, distance: i32) -> Self {
        self.distance = distance;
        self
    }

    fn diff(&self, chunks: &Vec<dissimilar::Chunk<'_>>) -> String {
        use std::fmt::Write;
        let mut f = String::with_capacity(chunks.len());
        for c in chunks {
            match *c {
                dissimilar::Chunk::Equal(ref s) => write!(f, "{}", s),
                dissimilar::Chunk::Delete(ref s) => write!(f, "\x1b[92m{}\x1b[0m", s),
                dissimilar::Chunk::Insert(ref s) => write!(f, "\x1b[91m{}\x1b[0m", s),
            }
            .expect("write to String")
        }
        f
    }
}

fn distance(chunks: &Vec<dissimilar::Chunk>) -> i32 {
    use dissimilar::Chunk::*;
    chunks.iter().fold(0, |n, c| match c {
        Equal(_) => n,
        _ => n + 1,
    })
}

impl Predicate<str> for DissimilarPredicate {
    fn eval(&self, edit: &str) -> bool {
        let chunks = dissimilar::diff(&self.orig, edit);
        self.op.eval(self.distance, distance(&chunks))
    }

    fn find_case<'a>(&'a self, expected: bool, variable: &str) -> Option<reflection::Case<'a>> {
        let chunks = dissimilar::diff(&self.orig, variable);
        let distance = distance(&chunks);
        let result = self.op.eval(self.distance, distance);
        if result == expected {
            Some(
                reflection::Case::new(Some(self), result)
                    .add_product(reflection::Product::new("distance", distance))
                    .add_product(reflection::Product::new("diff", self.diff(&chunks))),
            )
        } else {
            None
        }
    }
}

impl reflection::PredicateReflection for DissimilarPredicate {
    fn parameters<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Parameter<'a>> + 'a> {
        let params = vec![reflection::Parameter::new("original", &self.orig)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for DissimilarPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.op {
            DistanceOp::Similar => write!(f, "var - original <= {}", self.distance),
            DistanceOp::Different => write!(f, "{} < var - original", self.distance),
        }
    }
}

/// Creates a new `Predicate` that diffs two strings.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::diff2("Hello World");
/// assert_eq!(false, predicate_fn.eval("Hello World"));
/// assert_eq!(true, predicate_fn.eval("Goodbye World"));
/// ```
pub fn diff2<S>(orig: S) -> DissimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    DissimilarPredicate {
        orig: orig.into(),
        distance: 0,
        op: DistanceOp::Different,
    }
}

/// Creates a new `Predicate` that checks strings for how similar they are.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::similar2("Hello World");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Goodbye World"));
/// ```
pub fn similar2<S>(orig: S) -> DissimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    DissimilarPredicate {
        orig: orig.into(),
        distance: 0,
        op: DistanceOp::Similar,
    }
}
