// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[cfg(feature = "treeline")]
use treeline::Tree;

pub(crate) fn pass_fail(b: bool) -> &'static str {
    if b {
        "PASSED"
    } else {
        "FAILED"
    }
}

/// Trait for generically evaluating a type against a dynamically created
/// predicate function.
///
/// The exact meaning of `eval` depends on the situation, but will usually
/// mean that the evaluated item is in some sort of pre-defined set.  This is
/// different from `Ord` and `Eq` in that an `item` will almost never be the
/// same type as the implementing `Predicate` type.
pub trait Predicate<Item: ?Sized + fmt::Debug>: fmt::Display {
    /// Execute this `Predicate` against `variable`, returning the resulting
    /// boolean.
    fn eval(&self, variable: &Item) -> bool;

    /// TODO
    fn stringify(&self, _item: &Item) -> String {
        unimplemented!()
    }

    /// TODO
    #[cfg(feature = "treeline")]
    fn make_tree(&self, _item: &Item) -> Tree<String> {
        unimplemented!()
    }

    /// TODO
    #[cfg(feature = "treeline")]
    fn tree_eval(&self, item: &Item) -> (bool, Tree<String>) {
        (self.eval(item), self.make_tree(item))
    }
}
