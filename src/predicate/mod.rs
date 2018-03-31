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
mod function;
mod ord;
mod set;
pub use self::constant::{always, never, BooleanPredicate};
pub use self::function::{function, FnPredicate};
pub use self::ord::{eq, ge, gt, le, lt, ne, EqPredicate, OrdPredicate};
pub use self::set::{contains, contains_hashable, contains_ord, ContainsPredicate,
                    HashableContainsPredicate, OrdContainsPredicate};

// specialized primitive `Predicate` types
pub mod str;
pub mod path;

// combinators
mod boolean;
mod boxed;
pub use self::boolean::{AndPredicate, NotPredicate, OrPredicate};
pub use self::boxed::BoxPredicate;

/// Trait for generically evaluating a type against a dynamically created
/// predicate function.
///
/// The exact meaning of `eval` depends on the situation, but will usually
/// mean that the evaluated item is in some sort of pre-defined set.  This is
/// different from `Ord` and `Eq` in that an `item` will almost never be the
/// same type as the implementing `Predicate` type.
pub trait Predicate {
    /// The type that this `Predicate` will accept for evaluating.
    type Item: ?Sized;

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
    where
        B: Predicate<Item = Self::Item>,
        Self: Sized,
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
    where
        B: Predicate<Item = Self::Item>,
        Self: Sized,
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
    where
        Self: Sized,
    {
        NotPredicate::new(self)
    }

    /// Returns a `BoxPredicate` wrapper around this `Predicate` type.
    ///
    /// Returns a `BoxPredicate` wrapper around this `Predicate type. The
    /// `BoxPredicate` type has a number of useful properties:
    ///
    ///   - It stores the inner predicate as a trait object, so the type of
    ///     `BoxPredicate` will always be the same even if steps are added or
    ///     removed from the predicate.
    ///   - It is a common type, allowing it to be stored in vectors or other
    ///     collection types.
    ///   - It implements `Debug` and `Display`.
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
    fn boxed(self) -> BoxPredicate<Self::Item>
    where
        Self: Sized + Send + Sync + 'static,
    {
        BoxPredicate::new(self)
    }
}
