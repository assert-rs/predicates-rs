// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Path Predicates
//!
//! This module contains predicates specifiuc to the file system.

mod existence;
pub use self::existence::{exists, missing, ExistencePredicate};
