// Copyright (c) 2018 The predicates-rs Project Developers.
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
//! predicates = "0.5"
//! ```
//!
//! ## Examples
//!
//! A few different examples of how predicates might be used:
//!
//! ```
//! use predicates::prelude::*;
//!
//! use std::fmt;
//!
//! // The simplest predicates are `always()` and `never()`, which always returns
//! // `true` and always returns `false`, respectively. The values are simply
//! // ignored when evaluating against these predicates:
//! let always_true = predicate::always();
//! assert_eq!(true, always_true.eval(&5));
//! let always_false = predicate::never();
//! assert_eq!(false, always_false.eval(&5));
//!
//! // Pre-made predicates are available for types that implement the `PartialOrd` and
//! // `PartialEq` traits. The following example uses `lt`, but `eq`, `ne`, `le`, `gt`,
//! // `ge` are also available.
//! let less_than_ten = predicate::lt(10);
//! assert_eq!(true, less_than_ten.eval(&9));
//! assert_eq!(false, less_than_ten.eval(&11));
//!
//! // The `Predicate` type is actually a trait, and that trait implements a
//! // number of useful combinator functions. For example, evaluating for a value
//! // between two other values can be accomplished as follows:
//! let between_5_and_10 = predicate::ge(5).and(predicate::le(10));
//! assert_eq!(true, between_5_and_10.eval(&7));
//! assert_eq!(false, between_5_and_10.eval(&11));
//! assert_eq!(false, between_5_and_10.eval(&4));
//!
//! // The `Predicate` trait is pretty simple, the core of it is an
//! // implementation of a `eval` function that takes a single argument and
//! // returns a `bool`. Implementing a custom `Predicate` still allows all the
//! // usual combinators of the `Predicate` trait to work!
//! struct IsTheAnswer;
//! impl Predicate<i32> for IsTheAnswer {
//!     fn eval(&self, variable: &i32) -> bool {
//!         *variable == 42
//!     }
//! }
//! impl predicates::reflection::PredicateReflection for IsTheAnswer {}
//! impl fmt::Display for IsTheAnswer {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "var.is_the_answer()")
//!     }
//! }
//!
//! assert_eq!(true, IsTheAnswer.eval(&42));
//! let almost_the_answer = IsTheAnswer.or(predicate::in_iter(vec![41, 43]));
//! assert_eq!(true, almost_the_answer.eval(&41));
//!
//! // Any function over a reference to the desired `Item` that returns `bool`
//! // can easily be made into a `Predicate` using the `predicate::function`
//! // function.
//! let bound = 5;
//! let predicate_fn = predicate::function(|&x| x >= bound);
//! let between_5_and_10 = predicate_fn.and(predicate::le(10));
//! assert_eq!(true, between_5_and_10.eval(&7));
//! assert_eq!(false, between_5_and_10.eval(&3));
//! ```

#![warn(missing_docs, missing_debug_implementations)]

#[cfg(feature = "difference")]
extern crate difference;
#[cfg(feature = "float-cmp")]
extern crate float_cmp;
#[cfg(feature = "normalize-line-endings")]
extern crate normalize_line_endings;
#[cfg(feature = "regex")]
extern crate regex;

pub mod prelude;

mod core;
pub use core::*;
mod boxed;
pub use boxed::*;
pub mod reflection;

// core predicates
pub mod constant;
pub mod function;
pub mod iter;
pub mod name;
pub mod ord;

// combinators
pub mod boolean;

// specialized primitive `Predicate` types
pub mod float;
pub mod path;
pub mod str;
