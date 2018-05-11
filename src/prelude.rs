// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module that contains the essentials for working with predicates.

pub use core::Predicate;
pub use boolean::PredicateBooleanExt;
pub use boxed::PredicateBoxExt;
pub use str::PredicateStrExt;

/// Predicate factories
pub mod predicate {
    // primitive `Predicate` types
    pub use constant::{always, never};
    pub use function::function;
    pub use ord::{eq, ge, gt, le, lt, ne};
    pub use iter::{in_hash, in_iter};

    /// `str` Predicate factories
    ///
    /// This module contains predicates specific to string handling.
    pub mod str {
        pub use str::is_empty;
        pub use str::{contains, ends_with, starts_with};

        #[cfg(feature = "difference")]
        pub use str::{diff, similar};

        #[cfg(feature = "regex")]
        pub use str::is_match;
    }

    /// `Path` Predicate factories
    ///
    /// This module contains predicates specific to path handling.
    pub mod path {
        pub use path::{exists, missing};
        pub use path::{is_dir, is_file, is_symlink};
    }

    /// `f64` Predicate factories
    ///
    /// This module contains predicates specific to float handling.
    pub mod float {
        #[cfg(feature = "float-cmp")]
        pub use float::is_close;
    }
}
