// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of `Predicate` for wrapping a `Fn(&T) -> bool`

use std::marker::PhantomData;

use Predicate;

/// Predicate that wraps a function over a reference that returns a `bool`.
/// This type is returned by the `predicate::function` function.
#[derive(Debug)]
pub struct FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
{
    function: F,
    _phantom: PhantomData<T>,
}

impl<F, T> Predicate<T> for FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
{
    fn eval(&self, variable: &T) -> bool {
        (self.function)(variable)
    }
}

/// Creates a new predicate that wraps over the given function. The returned
/// type implements `Predicate` and therefore has all combinators available to
/// it.
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// struct Example {
///     string: String,
///     number: i32,
/// }
///
/// let string_check = predicate::function(|x: &Example| x.string == "hello");
/// let number_check = predicate::function(|x: &Example| x.number == 42);
/// let predicate_fn = string_check.and(number_check);
/// let good_example = Example { string: "hello".into(), number: 42 };
/// assert_eq!(true, predicate_fn.eval(&good_example));
/// let bad_example = Example { string: "goodbye".into(), number: 0 };
/// assert_eq!(false, predicate_fn.eval(&bad_example));
/// ```
pub fn function<F, T>(function: F) -> FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
{
    FnPredicate {
        function: function,
        _phantom: PhantomData,
    }
}
