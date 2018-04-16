// Copyright (c) 2018 The predicates-rs Project Developers.
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
/// set, otherwise returns `false`.
///
/// Note that this implementation places the fewest restrictions on the
/// underlying `Item` type at the expense of having the least performant
/// implementation (linear search). If the type to be searched is `Hash + Eq`,
/// it is much more efficient to use `HashableContainsPredicate` and
/// `contains_hashable`. The implementation-specific predicates will be
/// deprecated when Rust supports trait specialization.
#[derive(Debug)]
pub struct ContainsPredicate<T>
where
    T: PartialEq,
{
    inner: Vec<T>,
}

impl<T> Predicate for ContainsPredicate<T>
where
    T: PartialEq,
{
    type Item = T;

    fn eval(&self, variable: &Self::Item) -> bool {
        self.inner.contains(variable)
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// Note that this implementation places the fewest restrictions on the
/// underlying `Item` type at the expense of having the least performant
/// implementation (linear search). If the type to be searched is `Hash + Eq`,
/// it is much more efficient to use `HashableContainsPredicate` and
/// `contains_hashable`. The implementation-specific predicates will be
/// deprecated when Rust supports trait specialization.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::contains(vec![1, 3, 5]);
/// assert_eq!(true, predicate_fn.eval(&1));
/// assert_eq!(false, predicate_fn.eval(&2));
/// assert_eq!(true, predicate_fn.eval(&3));
/// assert_eq!(false, predicate_fn.eval(&4));
/// assert_eq!(true, predicate_fn.eval(&5));
/// ```
pub fn contains<I, T>(iter: I) -> ContainsPredicate<T>
where
    T: PartialEq,
    I: IntoIterator<Item = T>,
{
    ContainsPredicate {
        inner: Vec::from_iter(iter),
    }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// set, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Ord`. The
/// `ContainsPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::contains_ord` function.
#[derive(Debug)]
pub struct OrdContainsPredicate<T>
where
    T: Ord,
{
    inner: Vec<T>,
}

impl<T> Predicate for OrdContainsPredicate<T>
where
    T: Ord,
{
    type Item = T;

    fn eval(&self, variable: &Self::Item) -> bool {
        self.inner.binary_search(variable).is_ok()
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// Note that this implementation requires `Item` to be `Ord`. The
/// `ContainsPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::contains_ord(vec![1, 3, 5]);
/// assert_eq!(true, predicate_fn.eval(&1));
/// assert_eq!(false, predicate_fn.eval(&2));
/// assert_eq!(true, predicate_fn.eval(&3));
/// assert_eq!(false, predicate_fn.eval(&4));
/// assert_eq!(true, predicate_fn.eval(&5));
/// ```
pub fn contains_ord<I, T>(iter: I) -> OrdContainsPredicate<T>
where
    T: Ord,
    I: IntoIterator<Item = T>,
{
    let mut items = Vec::from_iter(iter);
    items.sort();
    OrdContainsPredicate { inner: items }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// `HashSet`, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Hash + Eq`. The
/// `ContainsPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::contains_hashable` function.
#[derive(Debug)]
pub struct HashableContainsPredicate<T>
where
    T: Hash + Eq,
{
    inner: HashSet<T>,
}

impl<T> Predicate for HashableContainsPredicate<T>
where
    T: Hash + Eq,
{
    type Item = T;

    fn eval(&self, variable: &Self::Item) -> bool {
        self.inner.contains(variable)
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// Note that this implementation requires `Item` to be `Hash + Eq`. The
/// `ContainsPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::contains_hashable(vec![1, 3, 5]);
/// assert_eq!(true, predicate_fn.eval(&1));
/// assert_eq!(false, predicate_fn.eval(&2));
/// assert_eq!(true, predicate_fn.eval(&3));
/// assert_eq!(false, predicate_fn.eval(&4));
/// assert_eq!(true, predicate_fn.eval(&5));
/// ```
pub fn contains_hashable<I, T>(iter: I) -> HashableContainsPredicate<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    HashableContainsPredicate {
        inner: HashSet::from_iter(iter),
    }
}
