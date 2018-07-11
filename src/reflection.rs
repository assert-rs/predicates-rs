// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Introspect into the state of a `Predicate`.

use std::fmt;

/// Introspect the state of a `Predicate`.
pub trait PredicateReflection: fmt::Display {
    /// Parameters of the current `Predicate`.
    fn parameters<'a>(&'a self) -> Box<Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![];
        Box::new(params.into_iter())
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<Iterator<Item = Child<'a>> + 'a> {
        let params = vec![];
        Box::new(params.into_iter())
    }
}

/// A view of a `Predicate` parameter, provided by reflection.
///
/// ```rust
/// use predicates;
///
/// let param = predicates::reflection::Parameter::new("key", &10);
/// println!("{}", param);
/// ```
pub struct Parameter<'a>(&'a str, &'a fmt::Display);

impl<'a> Parameter<'a> {
    /// Create a new `Parameter`.
    pub fn new(key: &'a str, value: &'a fmt::Display) -> Self {
        Self { 0: key, 1: value }
    }

    /// Access the `Parameter` name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Parameter` value.
    pub fn value(&self) -> &fmt::Display {
        self.1
    }
}

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}

/// A view of a `Predicate` child, provided by reflection.
pub struct Child<'a>(&'a str, &'a PredicateReflection);

impl<'a> Child<'a> {
    /// Create a new `Predicate` child.
    pub fn new(key: &'a str, value: &'a PredicateReflection) -> Self {
        Self { 0: key, 1: value }
    }

    /// Access the `Child`'s name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Child` `Predicate`.
    pub fn value(&self) -> &PredicateReflection {
        self.1
    }
}

impl<'a> fmt::Display for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}
