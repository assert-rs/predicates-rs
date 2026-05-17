// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Map input values before evaluating an inner predicate.

use std::fmt;
use std::marker::PhantomData;

use crate::Predicate;
use crate::reflection;

/// Predicate adapter that maps the input value before evaluation.
///
/// This is created by [`PredicateMapExt::map`].
pub struct MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped>,
    Item: ?Sized,
{
    inner: Inner,
    map: F,
    _phantom: PhantomData<fn(&Item) -> Mapped>,
}

impl<Inner, Item, Mapped, F> MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped>,
    Item: ?Sized,
{
    /// Create a new `MapPredicate` from an inner predicate and a mapping function.
    pub fn new(inner: Inner, map: F) -> Self {
        Self {
            inner,
            map,
            _phantom: PhantomData,
        }
    }
}

unsafe impl<Inner, Item, Mapped, F> Send for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped> + Send,
    F: Send,
    Item: ?Sized,
{
}

unsafe impl<Inner, Item, Mapped, F> Sync for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped> + Sync,
    F: Sync,
    Item: ?Sized,
{
}

impl<Inner, Item, Mapped, F> Predicate<Item> for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped>,
    F: Fn(&Item) -> Mapped,
    Item: ?Sized,
{
    fn eval(&self, variable: &Item) -> bool {
        let mapped = (self.map)(variable);
        self.inner.eval(&mapped)
    }

    fn find_case<'a>(&'a self, expected: bool, variable: &Item) -> Option<reflection::Case<'a>> {
        let mapped = (self.map)(variable);
        self.inner
            .find_case(expected, &mapped)
            .map(|child_case| reflection::Case::new(Some(self), expected).add_child(child_case))
    }
}

impl<Inner, Item, Mapped, F> reflection::PredicateReflection for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped>,
    Item: ?Sized,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Child<'a>> + 'a> {
        let params = vec![reflection::Child::new("predicate", &self.inner)];
        Box::new(params.into_iter())
    }
}

impl<Inner, Item, Mapped, F> fmt::Display for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped> + fmt::Display,
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<Inner, Item, Mapped, F> fmt::Debug for MapPredicate<Inner, Item, Mapped, F>
where
    Inner: Predicate<Mapped> + fmt::Debug,
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapPredicate")
            .field("inner", &self.inner)
            .finish()
    }
}

/// `Predicate` extension that maps the input value before evaluation.
pub trait PredicateMapExt {
    /// Map the input value before passing it to this predicate.
    ///
    /// This is useful when a predicate is defined on a field or projection of a
    /// larger type, for example filtering `Task` values using `predicate::in_hash`
    /// on `TaskId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use std::collections::HashSet;
    ///
    /// #[derive(Debug)]
    /// struct Task {
    ///     id: u32,
    /// }
    ///
    /// let ids: HashSet<u32> = [1, 2].into_iter().collect();
    /// let in_ids = predicate::in_hash(ids);
    /// let has_id = in_ids.map(|task: &Task| task.id);
    ///
    /// assert!(has_id.eval(&Task { id: 1 }));
    /// assert!(!has_id.eval(&Task { id: 3 }));
    /// ```
    fn map<Item, Mapped, F>(self, map: F) -> MapPredicate<Self, Item, Mapped, F>
    where
        Self: Sized + Predicate<Mapped>,
        Item: ?Sized,
        F: Fn(&Item) -> Mapped;
}

impl<Inner> PredicateMapExt for Inner {
    fn map<Item, Mapped, F>(self, map: F) -> MapPredicate<Self, Item, Mapped, F>
    where
        Self: Sized + Predicate<Mapped>,
        Item: ?Sized,
        F: Fn(&Item) -> Mapped,
    {
        MapPredicate::new(self, map)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::prelude::*;

    #[derive(Debug)]
    struct Task {
        id: u32,
    }

    #[test]
    fn map_in_hash() {
        let ids: HashSet<u32> = [1, 2].into_iter().collect();
        let has_id = predicate::in_hash(ids).map(|task: &Task| task.id);

        assert!(has_id.eval(&Task { id: 1 }));
        assert!(!has_id.eval(&Task { id: 3 }));
    }

    #[test]
    fn map_combines_with_boxed() {
        let ids: HashSet<u32> = [1].into_iter().collect();
        let has_id = predicate::in_hash(ids).map(|task: &Task| task.id);

        let predicate = predicate::always()
            .boxed()
            .and(has_id.not())
            .boxed();

        assert!(predicate.eval(&Task { id: 2 }));
        assert!(!predicate.eval(&Task { id: 1 }));
    }
}
