// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of boolean logic combinators over `Predicate`s.

use std::fmt;
use std::marker::PhantomData;

#[cfg(feature = "treeline")]
use treeline::Tree;

use Predicate;

/// Predicate that combines two `Predicate`s, returning the AND of the results.
///
/// This is created by the `Predicate::and` function.
#[derive(Debug)]
pub struct AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

impl<M1, M2, Item> AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    /// Create a new `AndPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> AndPredicate<M1, M2, Item> {
        AndPredicate {
            a: a,
            b: b,
            _phantom: PhantomData,
        }
    }
}

impl<M1, M2, Item> Predicate<Item> for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn eval(&self, item: &Item) -> bool {
        self.a.eval(item) && self.b.eval(item)
    }

    #[cfg(feature = "treeline")]
    fn make_tree(&self, item: &Item) -> Tree<String> {
        Tree::new(
            format!(
                "{} {}",
                self.stringify(item),
                ::core::pass_fail(self.eval(item))
            ),
            vec![
                self.a.make_tree(item),
                self.b.make_tree(item),
            ]
        )
    }

    fn stringify(&self, item: &Item) -> String {
        format!("{} && {}", self.a.stringify(item), self.b.stringify(item))
    }
}

impl<M1, M2, Item> fmt::Display for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} && {})", self.a, self.b)
    }
}

/// Predicate that combines two `Predicate`s, returning the OR of the results.
///
/// This is created by the `Predicate::or` function.
#[derive(Debug)]
pub struct OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

impl<M1, M2, Item> OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    /// Create a new `OrPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> OrPredicate<M1, M2, Item> {
        OrPredicate {
            a: a,
            b: b,
            _phantom: PhantomData,
        }
    }
}

impl<M1, M2, Item> Predicate<Item> for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn eval(&self, item: &Item) -> bool {
        self.a.eval(item) || self.b.eval(item)
    }
}

impl<M1, M2, Item> fmt::Display for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} || {})", self.a, self.b)
    }
}

/// Predicate that returns a `Predicate` taking the logical NOT of the result.
///
/// This is created by the `Predicate::not` function.
#[derive(Debug)]
pub struct NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    inner: M,
    _phantom: PhantomData<Item>,
}

impl<M, Item> NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    /// Create a new `NotPredicate` over predicate `inner`.
    pub fn new(inner: M) -> NotPredicate<M, Item> {
        NotPredicate {
            inner: inner,
            _phantom: PhantomData,
        }
    }
}

impl<M, Item> Predicate<Item> for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn eval(&self, item: &Item) -> bool {
        !self.inner.eval(item)
    }

    #[cfg(feature = "treeline")]
    fn make_tree(&self, item: &Item) -> Tree<String> {
        Tree::new(
            format!(
                "{} {}",
                self.stringify(item),
                ::core::pass_fail(self.eval(item))
            ),
            vec![self.inner.make_tree(item)]
        )
    }

    fn stringify(&self, item: &Item) -> String {
        format!("!({})", self.inner.stringify(item))
    }
}

impl<M, Item> fmt::Display for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(! {})", self.inner)
    }
}

/// `Predicate` extension that adds boolean logic.
pub trait PredicateBooleanExt<Item: ?Sized + fmt::Debug>
where
    Self: Predicate<Item>,
{
    /// Compute the logical AND of two `Predicate` results, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn1 = predicate::always().and(predicate::always());
    /// let predicate_fn2 = predicate::always().and(predicate::never());
    /// assert_eq!(true, predicate_fn1.eval(&4));
    /// assert_eq!(false, predicate_fn2.eval(&4));
    fn and<B>(self, other: B) -> AndPredicate<Self, B, Item>
    where
        B: Predicate<Item>,
        Self: Sized,
    {
        AndPredicate::new(self, other)
    }

    /// Compute the logical OR of two `Predicate` results, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn1 = predicate::always().or(predicate::always());
    /// let predicate_fn2 = predicate::always().or(predicate::never());
    /// let predicate_fn3 = predicate::never().or(predicate::never());
    /// assert_eq!(true, predicate_fn1.eval(&4));
    /// assert_eq!(true, predicate_fn2.eval(&4));
    /// assert_eq!(false, predicate_fn3.eval(&4));
    fn or<B>(self, other: B) -> OrPredicate<Self, B, Item>
    where
        B: Predicate<Item>,
        Self: Sized,
    {
        OrPredicate::new(self, other)
    }

    /// Compute the logical NOT of a `Predicate`, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn1 = predicate::always().not();
    /// let predicate_fn2 = predicate::never().not();
    /// assert_eq!(false, predicate_fn1.eval(&4));
    /// assert_eq!(true, predicate_fn2.eval(&4));
    fn not(self) -> NotPredicate<Self, Item>
    where
        Self: Sized,
    {
        NotPredicate::new(self)
    }
}

impl<P, Item> PredicateBooleanExt<Item> for P
where
    P: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
}
