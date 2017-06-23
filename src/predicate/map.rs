// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of `Predicate`s for mapping one value to another before
//! performing comparisons.

use std::marker::PhantomData;

use Predicate;

/// Predicate wrapper that executes a function over the input value before
/// evaluating the inner predicate.
#[derive(Debug)]
pub struct MapPredicate<P, F, T, U>
    where P: Predicate<Item = U>,
          F: Fn(&T) -> &U
{
    inner: P,
    transform: F,
    _phantom: PhantomData<T>,
}

impl<P, F, T, U> Predicate for MapPredicate<P, F, T, U>
    where P: Predicate<Item = U>,
          F: Fn(&T) -> &U
{
    type Item = T;

    fn eval(&self, item: &Self::Item) -> bool {
        self.inner.eval((self.transform)(item))
    }
}

/// Execute a predicate on U when given type T by providing a transorm
/// function on `&T -> &U`.
///
/// # Examples
///
/// ```
/// use predicates::predicate::*;
///
/// struct Example {
///     string: &'static str,
///     number: u32,
/// }
///
/// let example = Example { string: "hello", number: 42 };
/// let m1 = map(|x: &Example| &x.string, eq("hello"));
/// let m2 = map(|x: &Example| &x.number, eq(42));
/// assert_eq!(true, m1.and(m2).eval(&example));
/// ```
pub fn map<P, F, T, U>(transform: F, predicate: P) -> MapPredicate<P, F, T, U>
    where P: Predicate<Item = U>,
          F: Fn(&T) -> &U
{
    MapPredicate {
        inner: predicate,
        transform: transform,
        _phantom: PhantomData,
    }
}
