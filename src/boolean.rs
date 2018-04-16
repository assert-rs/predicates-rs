// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of boolean logic combinators over `Predicate`s.

use Predicate;

/// Predicate that combines two `Predicate`s, returning the AND of the results.
///
/// This is created by the `Predicate::and` function.
#[derive(Debug)]
pub struct AndPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    a: M1,
    b: M2,
}

impl<M1, M2> AndPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    /// Create a new `AndPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> AndPredicate<M1, M2> {
        AndPredicate { a: a, b: b }
    }
}

impl<M1, M2> Predicate for AndPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    type Item = M1::Item;

    fn eval(&self, item: &Self::Item) -> bool {
        self.a.eval(item) && self.b.eval(item)
    }
}

/// Predicate that combines two `Predicate`s, returning the OR of the results.
///
/// This is created by the `Predicate::or` function.
#[derive(Debug)]
pub struct OrPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    a: M1,
    b: M2,
}

impl<M1, M2> OrPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    /// Create a new `OrPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> OrPredicate<M1, M2> {
        OrPredicate { a: a, b: b }
    }
}

impl<M1, M2> Predicate for OrPredicate<M1, M2>
where
    M1: Predicate,
    M2: Predicate<Item = M1::Item>,
{
    type Item = M1::Item;

    fn eval(&self, item: &Self::Item) -> bool {
        self.a.eval(item) || self.b.eval(item)
    }
}

/// Predicate that returns a `Predicate` taking the logical NOT of the result.
///
/// This is created by the `Predicate::not` function.
#[derive(Debug)]
pub struct NotPredicate<M>
where
    M: Predicate,
{
    inner: M,
}

impl<M> NotPredicate<M>
where
    M: Predicate,
{
    /// Create a new `NotPredicate` over predicate `inner`.
    pub fn new(inner: M) -> NotPredicate<M> {
        NotPredicate { inner: inner }
    }
}

impl<M> Predicate for NotPredicate<M>
where
    M: Predicate,
{
    type Item = M::Item;

    fn eval(&self, item: &Self::Item) -> bool {
        !self.inner.eval(item)
    }
}
