// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of a constant (always true or always false) `Predicate`.

use std::fmt;
use std::marker::PhantomData;

use core;
use reflection;
use Predicate;

/// Predicate that always returns a constant (boolean) result.
///
/// This is created by the `predicate::always` and `predicate::never` functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanPredicate<Item> {
    retval: bool,
    _phantom: PhantomData<Item>,
}

impl<Item> Predicate<Item> for BooleanPredicate<Item> {
    fn eval(&self, _variable: &Item) -> bool {
        self.retval
    }

    fn find_case<'a>(&'a self, expected: bool, variable: &Item) -> Option<reflection::Case<'a>> {
        core::default_find_case(self, expected, variable)
    }
}

impl<Item> reflection::PredicateReflection for BooleanPredicate<Item> {
    fn parameters<'a>(&'a self) -> Box<Iterator<Item = reflection::Parameter<'a>> + 'a> {
        let params = vec![reflection::Parameter::new("value", &self.retval)];
        Box::new(params.into_iter())
    }
}

impl<Item> fmt::Display for BooleanPredicate<Item> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.retval)
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
pub fn always<Item>() -> BooleanPredicate<Item> {
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
pub fn never<Item>() -> BooleanPredicate<Item> {
    BooleanPredicate {
        retval: false,
        _phantom: PhantomData,
    }
}
