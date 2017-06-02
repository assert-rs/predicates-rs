// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Predicate
//!
//! This module contains the `Predicate` trait and a number of combinators for
//! the trait. See the crate docs, and the docs for `Predicate`, for full detail.

// primitive `Predicate` types
mod constant;
mod ord;
mod set;
pub use self::constant::{always, never, BooleanPredicate};
pub use self::ord::{eq, ne, lt, le, gt, ge, EqPredicate, OrdPredicate};
pub use self::set::{contains, ContainsPredicate, contains_ord, OrdContainsPredicate,
                    contains_hashable, HashableContainsPredicate};

// combinators
mod boolean;
pub use self::boolean::{AndPredicate, OrPredicate, NotPredicate};

/// Trait for generically evaluating a type against a dynamically created
/// predicate function.
///
/// The exact meaning of `eval` depends on the situation, but will usually
/// mean that the evaluated item is in some sort of pre-defined set.  This is
/// different from `Ord` and `Eq` in that an `item` will almost never be the
/// same type as the implementing `Predicate` type.
pub trait Predicate {
    /// The type that this `Predicate` will accept for evaluating.
    type Item;

    /// Execute this `Predicate` against `variable`, returning the resulting
    /// boolean.
    fn eval(&self, variable: &Self::Item) -> bool;

    /// Compute the logical AND of two `Predicate` results, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let predicate_fn1 = always().and(always());
    /// let predicate_fn2 = always().and(never());
    /// assert_eq!(true, predicate_fn1.eval(&4));
    /// assert_eq!(false, predicate_fn2.eval(&4));
    fn and<B>(self, other: B) -> AndPredicate<Self, B>
        where B: Predicate<Item = Self::Item>,
              Self: Sized
    {
        AndPredicate::new(self, other)
    }

    /// Compute the logical OR of two `Predicate` results, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let predicate_fn1 = always().or(always());
    /// let predicate_fn2 = always().or(never());
    /// let predicate_fn3 = never().or(never());
    /// assert_eq!(true, predicate_fn1.eval(&4));
    /// assert_eq!(true, predicate_fn2.eval(&4));
    /// assert_eq!(false, predicate_fn3.eval(&4));
    fn or<B>(self, other: B) -> OrPredicate<Self, B>
        where B: Predicate<Item = Self::Item>,
              Self: Sized
    {
        OrPredicate::new(self, other)
    }

    /// Compute the logical NOT of a `Predicate`, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let predicate_fn1 = always().not();
    /// let predicate_fn2 = never().not();
    /// assert_eq!(false, predicate_fn1.eval(&4));
    /// assert_eq!(true, predicate_fn2.eval(&4));
    fn not(self) -> NotPredicate<Self>
        where Self: Sized
    {
        NotPredicate::new(self)
    }

    /// Convenience function that returns a trait object of this `Predicate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let predicates = vec![
    ///     always().boxed(),
    ///     never().boxed(),
    /// ];
    /// assert_eq!(true, predicates[0].eval(&4));
    /// assert_eq!(false, predicates[1].eval(&4));
    /// ```
    fn boxed(self) -> Box<Predicate<Item = Self::Item>>
        where Self: 'static + Sized
    {
        Box::new(self)
    }
}
