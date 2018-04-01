// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use float_cmp::ApproxEq;
use float_cmp::Ulps;

use Predicate;

/// Predicate that ensures two numbers are "close" enough, understanding that rounding errors
/// occur.
///
/// This is created by the `predicate::float::is_close`.
#[derive(Clone, Debug)]
pub struct IsClosePredicate {
    target: f64,
    epsilon: f64,
    ulps: <f64 as Ulps>::U,
}

impl IsClosePredicate {
    /// Set the amount of error allowed.
    ///
    /// Values `1`-`5` should work in most cases.  Some times more control is needed and you will
    /// need to set `IsClosePredicate::epsilon` separately from `IsClosePredicate::ulps`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let a = 0.15_f64 + 0.15_f64 + 0.15_f64;
    /// let predicate_fn = float::is_close(a).distance(5);
    /// ```
    pub fn distance(mut self, distance: <f64 as Ulps>::U) -> Self {
        self.epsilon = (distance as f64) * ::std::f64::EPSILON;
        self.ulps = distance;
        self
    }

    /// Set the absolute deviation allowed.
    ///
    /// This is meant to handle problems near `0`. Values `1.`-`5.` epislons should work in most
    /// cases.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let a = 0.15_f64 + 0.15_f64 + 0.15_f64;
    /// let predicate_fn = float::is_close(a).epsilon(5.0 * ::std::f64::EPSILON);
    /// ```
    pub fn epsilon(mut self, epsilon: f64) -> Self {
        self.epsilon = epsilon;
        self
    }

    /// Set the relative deviation allowed.
    ///
    /// This is meant to handle large numbers. Values `1`-`5` should work in most cases.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::predicate::*;
    ///
    /// let a = 0.15_f64 + 0.15_f64 + 0.15_f64;
    /// let predicate_fn = float::is_close(a).ulps(5);
    /// ```
    pub fn ulps(mut self, ulps: <f64 as Ulps>::U) -> Self {
        self.ulps = ulps;
        self
    }
}

impl Predicate for IsClosePredicate {
    type Item = f64;

    fn eval(&self, variable: &f64) -> bool {
        variable.approx_eq(&self.target, self.epsilon, self.ulps)
    }
}

/// Create a new `Predicate` that ensures two numbers are "close" enough, understanding that
/// rounding errors occur.
///
/// # Examples
///
/// ```
/// use predicates::predicate::*;
///
/// let a = 0.15_f64 + 0.15_f64 + 0.15_f64;
/// let b = 0.1_f64 + 0.1_f64 + 0.25_f64;
/// let predicate_fn = float::is_close(a);
/// assert_eq!(true, predicate_fn.eval(&b));
/// assert_eq!(false, predicate_fn.distance(0).eval(&b));
/// ```
pub fn is_close(target: f64) -> IsClosePredicate {
    IsClosePredicate {
        target,
        epsilon: 2.0 * ::std::f64::EPSILON,
        ulps: 2,
    }
}
