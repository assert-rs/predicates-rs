// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

struct TreeWriter<'a, Item: ?Sized + fmt::Debug + 'a> {
    items: Vec<&'a Predicate<Item>>,
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
    fn flatten<'a, 'b>(&'a self, _vec: &'b mut Vec<&'a Predicate<Item>>) {
        unimplemented!()
    }

    /// TODO
    fn stringify(&self, _variable: &Item) -> String {
        unimplemented!()
    }

    /// TODO
    #[cfg(feature = "term-table")]
    fn tree_eval(&self, variable: &Item) -> String {
        use term_table::{
            Table,
            cell::Cell,
            row::Row,
        };

        let mut table = Table::new();
        table.max_column_width = 80;
        let mut vec = Vec::new();
        self.flatten(&mut vec);

        table.add_row(Row::new(vec![
            Cell::new("PREDICATE", 1),
            Cell::new("RESULT", 1),
        ]));

        for item in vec {
            let result = if item.eval(variable) {"PASSED"} else {"FAILED"};

            table.add_row(Row::new(vec![
                Cell::new(item.stringify(variable), 1),
                Cell::new(result, 1),
            ]));
        }

        table.as_string()
    }
}
