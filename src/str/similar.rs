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
pub struct SimilarPredicate {
    old: borrow::Cow<'static, str>,
    algorithm: similar::Algorithm,
    distance: i32,
    op: DistanceOp,
}

impl SimilarPredicate {
    /// The maximum allowed edit distance.
    ///
    /// Default: `0`
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::similar3("Hello World!").distance(1);
    /// assert_eq!(true, predicate_fn.eval("Hello World!"));
    /// assert_eq!(true, predicate_fn.eval("Hello World"));
    ///
    /// assert_eq!(false, predicate_fn.eval("Hello... World?"));
    /// let dist = predicate_fn.find_case(false, "Hello... World?").unwrap().product_value("distance").unwrap();
    /// assert_eq!("2", &dist);
    /// ```
    pub fn distance(mut self, distance: i32) -> Self {
        self.distance = distance;
        self
    }

    fn new(old: borrow::Cow<'static, str>, op: DistanceOp) -> Self {
        Self{old, algorithm: similar::Algorithm::Myers, distance: 0, op}
    }

    fn diff(&self, chunks: &Vec<similar::DiffOp>) -> String {
        use std::fmt::Write;
        let mut f = String::with_capacity(chunks.len());
        for c in chunks {
            match *c {
                similar::DiffOp::Equal{old_index, ..} => write!(f, "{}", old_index),
                similar::DiffOp::Delete{old_index, ..} => write!(f, "\x1b[92m{}\x1b[0m", old_index),
                similar::DiffOp::Insert{old_index, ..} => write!(f, "\x1b[91m{}\x1b[0m", old_index),
                similar::DiffOp::Replace{old_index, ..} => write!(f, "\x1b[91m{}\x1b[0m", old_index),
            }
            .expect("write to String")
        }
        f
    }
}

// TODO `similar offers a diff_ratio which might be better
// TODO s/distance/changes/ ? Reserve "distance" for some levenstein-like metric ?
fn distance(chunks: &Vec<similar::DiffOp>) -> i32 {
    use similar::DiffOp::*;
    chunks.iter().fold(0, |n, c| match c {
        Equal{..} => n,
        _ => n + 1,
    })
}

impl Predicate<str> for SimilarPredicate {
    fn eval(&self, new: &str) -> bool {
        let chunks = similar::capture_diff_slices(self.algorithm, self.old.as_bytes(), new.as_bytes());
        self.op.eval(self.distance, distance(&chunks))
    }

    fn find_case<'a>(&'a self, expected: bool, new: &str) -> Option<reflection::Case<'a>> {
        let chunks = similar::capture_diff_slices(self.algorithm, self.old.as_bytes(), new.as_bytes());
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

impl reflection::PredicateReflection for SimilarPredicate {
    fn parameters<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Parameter<'a>> + 'a> {
        let params = vec![reflection::Parameter::new("original", &self.old)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for SimilarPredicate {
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
/// let predicate_fn = predicate::str::diff3("Hello World");
/// assert_eq!(false, predicate_fn.eval("Hello World"));
/// assert_eq!(true, predicate_fn.eval("Goodbye World"));
/// ```
pub fn diff3<S>(old: S) -> SimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    SimilarPredicate::new(old.into(), DistanceOp::Different)
}

/// Creates a new `Predicate` that checks strings for how similar they are.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::similar3("Hello World");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Goodbye World"));
/// ```
pub fn similar3<S>(old: S) -> SimilarPredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    SimilarPredicate::new(old.into(), DistanceOp::Similar)
}
