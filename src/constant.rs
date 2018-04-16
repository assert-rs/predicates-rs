// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of a constant (always true or always false) `Predicate`.

use std::marker::PhantomData;

use Predicate;

/// Predicate that always returns a constant (boolean) result.
///
/// This is created by the `predicate::always` and `predicate::never` functions.
#[derive(Debug)]
pub struct BooleanPredicate<T> {
    retval: bool,
    _phantom: PhantomData<T>,
}

impl<T> Predicate for BooleanPredicate<T> {
    type Item = T;

    fn eval(&self, _variable: &T) -> bool {
        self.retval
    }
}

/// Creates a new `Predicate` that always returns `true`.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::always();
/// assert_eq!(true, predicate_fn.eval(&5));
/// assert_eq!(true, predicate_fn.eval(&10));
/// assert_eq!(true, predicate_fn.eval(&15));
/// // Won't work - Predicates can only operate on a single type
/// // assert_eq!(true, predicate_fn.eval("hello"))
/// ```
pub fn always<T>() -> BooleanPredicate<T> {
    BooleanPredicate {
        retval: true,
        _phantom: PhantomData,
    }
}

/// Creates a new `Predicate` that always returns `false`.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::never();
/// assert_eq!(false, predicate_fn.eval(&5));
/// assert_eq!(false, predicate_fn.eval(&10));
/// assert_eq!(false, predicate_fn.eval(&15));
/// // Won't work - Predicates can only operate on a single type
/// // assert_eq!(false, predicate_fn.eval("hello"))
/// ```
pub fn never<T>() -> BooleanPredicate<T> {
    BooleanPredicate {
        retval: false,
        _phantom: PhantomData,
    }
}
