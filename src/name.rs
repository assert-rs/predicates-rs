// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Name predicate expressions.

use std::fmt;
use std::marker::PhantomData;

use Predicate;

/// Augment an existing predicate with a name.
///
/// This is created by the `PredicateNameExt::name` function.
#[derive(Debug)]
pub struct NamePredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    inner: M,
    name: &'static str,
    _phantom: PhantomData<Item>,
}

impl<M, Item> Predicate<Item> for NamePredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn eval(&self, item: &Item) -> bool {
        self.inner.eval(item)
    }
}

impl<M, Item> fmt::Display for NamePredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// `Predicate` extension that adds naming predicate expressions.
pub trait PredicateNameExt<Item: ?Sized + fmt::Debug>
where
    Self: Predicate<Item>,
{
    /// Name a predicate expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::is_empty().not().name("non-empty");
    /// println!("{}", predicate_fn);
    /// ```
    fn name(self, name: &'static str) -> NamePredicate<Self, Item>
    where
        Self: Sized,
    {
        NamePredicate {
            inner: self,
            name,
            _phantom: PhantomData,
        }
    }
}

impl<P, Item> PredicateNameExt<Item> for P
where
    P: Predicate<Item>,
    Item: ?Sized + fmt::Debug,
{
}
