// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of `Predicate`s for comparisons of membership in a set.

use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

use Predicate;

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// set, otherwise returns `false`.
///
/// Note that this implementation places the fewest restrictions on the
/// underlying `Item` type at the expense of having the least performant
/// implementation (linear search). If the type to be searched is `Hash + Eq`,
/// it is much more efficient to use `HashableInPredicate` and
/// `in_hash`. The implementation-specific predicates will be
/// deprecated when Rust supports trait specialization.
#[derive(Debug)]
pub struct InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    inner: Vec<T>,
}

impl<T> InPredicate<T>
where
    T: Ord + fmt::Debug,
{
    /// Creates a new predicate that will return `true` when the given `variable` is
    /// contained with the set of items provided.
    ///
    /// Note that this implementation requires `Item` to be `Ord`. The
    /// `InPredicate` uses a less efficient search algorithm but only
    /// requires `Item` implement `PartialEq`. The implementation-specific
    /// predicates will be deprecated when Rust supports trait specialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::in_iter(vec![1, 3, 5]).sort();
    /// assert_eq!(true, predicate_fn.eval(&1));
    /// assert_eq!(false, predicate_fn.eval(&2));
    /// assert_eq!(true, predicate_fn.eval(&3));
    /// assert_eq!(false, predicate_fn.eval(&4));
    /// assert_eq!(true, predicate_fn.eval(&5));
    /// ```
    pub fn sort(self) -> OrdInPredicate<T> {
        let mut items = self.inner;
        items.sort();
        OrdInPredicate { inner: items }
    }
}

impl<T> Predicate<T> for InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    fn eval(&self, variable: &T) -> bool {
        self.inner.contains(variable)
    }
}

impl<T> fmt::Display for InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var in {:?}", self.inner)
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// Note that this implementation places the fewest restrictions on the
/// underlying `Item` type at the expense of having the least performant
/// implementation (linear search). If the type to be searched is `Hash + Eq`,
/// it is much more efficient to use `HashableInPredicate` and
/// `in_hash`. The implementation-specific predicates will be
/// deprecated when Rust supports trait specialization.
///
/// If you need to optimize this
/// - Type is `Ord`, call `sort()` on this predicate.
/// - Type is `Hash`, replace `in_iter` with `in_hash`.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::in_iter(vec![1, 3, 5]);
/// assert_eq!(true, predicate_fn.eval(&1));
/// assert_eq!(false, predicate_fn.eval(&2));
/// assert_eq!(true, predicate_fn.eval(&3));
/// assert_eq!(false, predicate_fn.eval(&4));
/// assert_eq!(true, predicate_fn.eval(&5));
/// ```
pub fn in_iter<I, T>(iter: I) -> InPredicate<T>
where
    T: PartialEq + fmt::Debug,
    I: IntoIterator<Item = T>,
{
    InPredicate {
        inner: Vec::from_iter(iter),
    }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// set, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Ord`. The
/// `InPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::in_iter(...).sort` function.
#[derive(Debug)]
pub struct OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    inner: Vec<T>,
}

impl<T> Predicate<T> for OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    fn eval(&self, variable: &T) -> bool {
        self.inner.binary_search(variable).is_ok()
    }
}

impl<T> fmt::Display for OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var in {:?}", self.inner)
    }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// `HashSet`, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Hash + Eq`. The
/// `InPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::in_hash` function.
#[derive(Debug)]
pub struct HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    inner: HashSet<T>,
}

impl<T> Predicate<T> for HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    fn eval(&self, variable: &T) -> bool {
        self.inner.contains(variable)
    }
}

impl<T> fmt::Display for HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var in {:?}", self.inner)
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
///
/// Note that this implementation requires `Item` to be `Hash + Eq`. The
/// `InPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::in_hash(vec![1, 3, 5]);
/// assert_eq!(true, predicate_fn.eval(&1));
/// assert_eq!(false, predicate_fn.eval(&2));
/// assert_eq!(true, predicate_fn.eval(&3));
/// assert_eq!(false, predicate_fn.eval(&4));
/// assert_eq!(true, predicate_fn.eval(&5));
/// ```
pub fn in_hash<I, T>(iter: I) -> HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
    I: IntoIterator<Item = T>,
{
    HashableInPredicate {
        inner: HashSet::from_iter(iter),
    }
}
