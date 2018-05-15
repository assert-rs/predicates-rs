use std::ffi;
use std::fmt;
use std::str;

use Predicate;

/// Predicate adaper that trims the variable being tested.
///
/// This is created by `pred.trim()`.
#[derive(Copy, Clone, Debug)]
pub struct TrimPedicate<P>
where
    P: Predicate<str>,
{
    p: P,
}

impl<P> Predicate<str> for TrimPedicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &str) -> bool {
        self.p.eval(variable.trim())
    }
}

impl<P> fmt::Display for TrimPedicate<P>
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
#[derive(Copy, Clone, Debug)]
pub struct Utf8Pedicate<P>
where
    P: Predicate<str>,
{
    p: P,
}

impl<P> Predicate<ffi::OsStr> for Utf8Pedicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &ffi::OsStr) -> bool {
        variable.to_str().map(|s| self.p.eval(s)).unwrap_or(false)
    }
}

impl<P> Predicate<[u8]> for Utf8Pedicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &[u8]) -> bool {
        str::from_utf8(variable)
            .map(|s| self.p.eval(s))
            .unwrap_or(false)
    }
}

impl<P> fmt::Display for Utf8Pedicate<P>
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
    /// Returns a `TrimPedicate` that ensures the data passed to `Self` is trimmed.
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
    fn trim(self) -> TrimPedicate<Self> {
        TrimPedicate { p: self }
    }

    /// Returns a `Utf8Pedicate` that adapts `Self` to a `[u8]` `Predicate`.
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
    fn from_utf8(self) -> Utf8Pedicate<Self> {
        Utf8Pedicate { p: self }
    }
}

impl<P> PredicateStrExt for P
where
    P: Predicate<str>,
{
}
