// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
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
pub struct BoxPredicate<T: ?Sized>(Box<Predicate<Item = T> + Send + Sync>);

impl<T> fmt::Debug for BoxPredicate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxPredicate").finish()
    }
}

impl<T> fmt::Display for BoxPredicate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxPredicate")
    }
}

impl<T: ?Sized> Predicate for BoxPredicate<T> {
    type Item = T;

    fn eval(&self, variable: &Self::Item) -> bool {
        self.0.eval(variable)
    }
}

impl<T: ?Sized> BoxPredicate<T> {
    /// Creates a new `BoxPredicate`, a wrapper around a dynamically-dispatched
    /// `Predicate` type with useful trait impls.
    pub fn new<P: Predicate<Item = T>>(inner: P) -> BoxPredicate<T>
    where
        P: Send + Sync + 'static,
    {
        BoxPredicate(Box::new(inner))
    }
}
