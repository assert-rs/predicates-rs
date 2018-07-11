// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi;
use std::fmt;
use std::str;

use reflection;
use Predicate;
#[cfg(feature = "normalize-line-endings")]
use str::normalize::NormalizedPredicate;

/// Predicate adaper that trims the variable being tested.
///
/// This is created by `pred.trim()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimPredicate<P>
where
    P: Predicate<str>,
{
    p: P,
}

impl<P> Predicate<str> for TrimPredicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &str) -> bool {
        self.p.eval(variable.trim())
    }
}

impl<P> reflection::PredicateReflection for TrimPredicate<P>
where
    P: Predicate<str>,
{
}

impl<P> fmt::Display for TrimPredicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.p)
    }
}

/// Predicate adaper that converts a `str` predicate to byte predicate.
///
/// This is created by `pred.from_utf8()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Predicate<P>
where
    P: Predicate<str>,
{
    p: P,
}

impl<P> Predicate<ffi::OsStr> for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &ffi::OsStr) -> bool {
        variable.to_str().map(|s| self.p.eval(s)).unwrap_or(false)
    }
}

impl<P> Predicate<[u8]> for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &[u8]) -> bool {
        str::from_utf8(variable)
            .map(|s| self.p.eval(s))
            .unwrap_or(false)
    }
}

impl<P> reflection::PredicateReflection for Utf8Predicate<P>
where
    P: Predicate<str>,
{
}

impl<P> fmt::Display for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.p)
    }
}

/// `Predicate` extension adapting a `str` Predicate.
pub trait PredicateStrExt
where
    Self: Predicate<str>,
    Self: Sized,
{
    /// Returns a `TrimPredicate` that ensures the data passed to `Self` is trimmed.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::is_empty().trim();
    /// assert_eq!(true, predicate_fn.eval("    "));
    /// assert_eq!(false, predicate_fn.eval("    Hello    "));
    /// ```
    fn trim(self) -> TrimPredicate<Self> {
        TrimPredicate { p: self }
    }

    /// Returns a `Utf8Predicate` that adapts `Self` to a `[u8]` `Predicate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use std::ffi::OsStr;
    ///
    /// let predicate_fn = predicate::str::is_empty().not().from_utf8();
    /// assert_eq!(true, predicate_fn.eval(OsStr::new("Hello")));
    /// assert_eq!(false, predicate_fn.eval(OsStr::new("")));
    /// let variable: &[u8] = b"";
    /// assert_eq!(false, predicate_fn.eval(variable));
    /// ```
    fn from_utf8(self) -> Utf8Predicate<Self> {
        Utf8Predicate { p: self }
    }

    /// Returns a `NormalizedPredicate` that ensures
    ///  the newlines within the data passed to `Self` is normalised.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// 
    /// let predicate_fn = predicate::eq("Hello World!\n").normalize();
    /// assert_eq!(true, predicate_fn.eval("Hello World!\n"));
    /// assert_eq!(true, predicate_fn.eval("Hello World!\r"));
    /// assert_eq!(true, predicate_fn.eval("Hello World!\r\n"));
    /// assert_eq!(false, predicate_fn.eval("Goodbye"));
    /// ```
    ///
    #[cfg(feature = "normalize-line-endings")]
    fn normalize(self) -> NormalizedPredicate<Self> {
        NormalizedPredicate { p: self }
    }

}

impl<P> PredicateStrExt for P
where
    P: Predicate<str>,
{
}
