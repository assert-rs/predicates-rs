// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! String Predicates
//!
//! This module contains predicates specific to string handling.

mod basics;
pub use self::basics::*;
mod adapters;
pub use self::adapters::*;

#[cfg(feature = "difference")]
mod difference;
#[cfg(feature = "difference")]
pub use self::difference::{diff, similar, DifferencePredicate};

#[cfg(feature = "dissimilar")]
mod dissimilar;
#[cfg(feature = "dissimilar")]
pub use self::dissimilar::{diff2, similar2, DissimilarPredicate};

#[cfg(feature = "similar")]
mod similar;
#[cfg(feature = "similar")]
pub use self::similar::{diff3, similar3, SimilarPredicate};

#[cfg(feature = "normalize-line-endings")]
mod normalize;
#[cfg(feature = "normalize-line-endings")]
pub use self::normalize::NormalizedPredicate;

#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "regex")]
pub use self::regex::{is_match, RegexError, RegexPredicate};
