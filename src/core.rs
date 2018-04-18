// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use boolean::{AndPredicate, NotPredicate, OrPredicate};
use boxed::BoxPredicate;

/// Trait for generically evaluating a type against a dynamically created
/// predicate function.
///
/// The exact meaning of `eval` depends on the situation, but will usually
/// mean that the evaluated item is in some sort of pre-defined set.  This is
/// different from `Ord` and `Eq` in that an `item` will almost never be the
/// same type as the implementing `Predicate` type.
pub trait Predicate<Item: ?Sized> {
    /// Execute this `Predicate` against `variable`, returning the resulting
    /// boolean.
    fn eval(&self, variable: &Item) -> bool;

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
    /// use predicates::prelude::*;
    ///
    /// let predicates = vec![
    ///     predicate::always().boxed(),
    ///     predicate::never().boxed(),
    /// ];
    /// assert_eq!(true, predicates[0].eval(&4));
    /// assert_eq!(false, predicates[1].eval(&4));
    /// ```
    fn boxed(self) -> BoxPredicate<Item>
    where
        Self: Sized + Send + Sync + 'static,
    {
        BoxPredicate::new(self)
    }
}
