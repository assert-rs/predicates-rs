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
}

impl<P> PredicateStrExt for P
where
    P: Predicate<str>,
{
}
