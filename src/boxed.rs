// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Predicate that can wrap other dynamically-called predicates in an
//! easy-to-manage type.

use std::fmt;

use Predicate;

/// `Predicate` that wraps another `Predicate` as a trait object, allowing
/// sized storage of predicate types.
pub struct BoxPredicate<Item: ?Sized + fmt::Debug>(Box<Predicate<Item> + Send + Sync>);

impl<Item> BoxPredicate<Item>
where
    Item: ?Sized + fmt::Debug,
{
    /// Creates a new `BoxPredicate`, a wrapper around a dynamically-dispatched
    /// `Predicate` type with useful trait impls.
    pub fn new<P: Predicate<Item>>(inner: P) -> BoxPredicate<Item>
    where
        P: Send + Sync + 'static,
    {
        BoxPredicate(Box::new(inner))
    }
}

impl<Item> fmt::Debug for BoxPredicate<Item>
where
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxPredicate").finish()
    }
}

impl<Item> fmt::Display for BoxPredicate<Item>
where
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Item> Predicate<Item> for BoxPredicate<Item>
where
    Item: ?Sized + fmt::Debug,
{
    fn eval(&self, variable: &Item) -> bool {
        self.0.eval(variable)
    }
}

/// `Predicate` extension for boxing a `Predicate`.
pub trait PredicateBoxExt<Item: ?Sized + fmt::Debug>
where
    Self: Predicate<Item>,
{
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

impl<P, Item> PredicateBoxExt<Item> for P
where
    P: Predicate<Item>,
    Item: ?Sized + fmt::Debug
{
}
