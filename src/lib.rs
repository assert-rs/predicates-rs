// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Composable first-order predicate functions.
//!
//! This library implements an interface to "predicates" - boolean-valued
//! functions of one argument. This allows combinatorial logic to be created and
//! assembled at runtime and then used one or more times for evaluating values.
//! This sort of object is really useful when creating filters and checks that
//! can be changed at runtime with user interaction - it allows a clean
//! separation of concerns where the configuration code can be used to build up
//! a predicate, and then that predicate can be given to the code that does the
//! actual filtering without the filtering code knowing anything about user
//! configuration. See the examples for how this can work.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! predicates = "0.1"
//! ```
//!
//! ## Examples
//!
//! A few different examples of how predicates might be used:
//!
//! ```
//! use predicates::{predicate, Predicate};
//!
//! // The simplest predicates are `always()` and `never()`, which always returns
//! // `true` and always returns `false`, respectively. The values are simply
//! // ignored when evaluating against these predicates:
//! let always_true = predicate::always();
//! assert!(always_true.eval(&5));
//! let always_false = predicate::never();
//! assert!(!always_false.eval(&5));
//!
//! // Pre-made predicates are available for types that implement the `PartialOrd` and
//! // `PartialEq` traits. The following example uses `lt`, but `eq`, `ne`, `le`, `gt`,
//! // `ge` are also available.
//! let less_than_ten = predicate::lt(10);
//! assert!(less_than_ten.eval(&9));
//! assert!(!less_than_ten.eval(&11));
//!
//! // The `Predicate` type is actually a trait, and that trait implements a
//! // number of useful combinator functions. For example, evaluating for a value
//! // between two other values can be accomplished as follows:
//! let between_5_and_10 = predicate::ge(5).and(predicate::le(10));
//! assert!(between_5_and_10.eval(&7));
//! assert!(!between_5_and_10.eval(&11));
//! assert!(!between_5_and_10.eval(&4));
//!
//! // The `Predicate` trait is pretty simple, requiring only the
//! // implementation of a `eval` function that takes a single argument and
//! // returns a `bool`. Implementing a custom `Predicate` still allows all the
//! // usual combinators of the `Predicate` trait to work!
//! struct IsTheAnswer;
//! impl Predicate for IsTheAnswer {
//!     type Item = i32;
//!     fn eval(&self, variable: &Self::Item) -> bool {
//!         *variable == 42
//!     }
//! }
//!
//! assert!(IsTheAnswer.eval(&42));
//! let almost_the_answer = IsTheAnswer.or(predicate::in_set(vec![41, 43]));
//! assert!(almost_the_answer.eval(&41));
//! ```

// core `Predicate` trait
pub mod predicate;
pub use self::predicate::Predicate;
