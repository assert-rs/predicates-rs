// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of `Predicate`s for comparisons of membership in a set.

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

use Predicate;

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// `HashSet`, otherwise returns `false`.
///
/// This is created by the `predicate::in_set` function.
#[derive(Debug)]
pub struct SetPredicate<T>
    where T: Hash + Eq
{
    inner: HashSet<T>,
}

impl<T> Predicate for SetPredicate<T>
    where T: Hash + Eq
{
    type Item = T;

    fn eval(&self, variable: &Self::Item) -> bool {
        self.inner.contains(variable)
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// # Examples
///
/// ```
/// use predicates::predicate::*;
///
/// let predicate_fn = in_set(vec![1, 3, 5]);
/// assert!(predicate_fn.eval(&1));
/// assert!(!predicate_fn.eval(&2));
/// assert!(predicate_fn.eval(&3));
/// assert!(!predicate_fn.eval(&4));
/// assert!(predicate_fn.eval(&5));
/// ```
pub fn in_set<I, T>(iter: I) -> SetPredicate<T>
    where T: Hash + Eq,
          I: IntoIterator<Item = T>
{
    SetPredicate { inner: HashSet::from_iter(iter) }
}
