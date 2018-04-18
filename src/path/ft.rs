// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path;
use std::fs;

use Predicate;

#[derive(Clone, Copy, Debug)]
enum FileType {
    File,
    Dir,
    Symlink,
}

impl FileType {
    fn eval(self, ft: &fs::FileType) -> bool {
        match self {
            FileType::File => ft.is_file(),
            FileType::Dir => ft.is_dir(),
            FileType::Symlink => ft.is_symlink(),
        }
    }
}

/// Predicate that checks the `std::fs::FileType`.
///
/// This is created by the `predicate::path::is_file`, `predicate::path::is_dir`, and `predicate::path::is_symlink`.
#[derive(Debug)]
pub struct FileTypePredicate {
    ft: FileType,
    follow: bool,
}

impl FileTypePredicate {
    /// Follow symbolic links.
    ///
    /// When yes is true, symbolic links are followed as if they were normal directories and files.
    ///
    /// Default: disabled.
    pub fn follow_links(mut self, yes: bool) -> Self {
        self.follow = yes;
        self
    }
}

impl Predicate<path::Path> for FileTypePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        let metadata = if self.follow {
            path.metadata()
        } else {
            path.symlink_metadata()
        };
        metadata
            .map(|m| self.ft.eval(&m.file_type()))
            .unwrap_or(false)
    }
}

/// Creates a new `Predicate` that ensures the path points to a file.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::path::is_file();
/// assert_eq!(true, predicate_fn.eval(Path::new("Cargo.toml")));
/// assert_eq!(false, predicate_fn.eval(Path::new("src")));
/// assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
/// ```
pub fn is_file() -> FileTypePredicate {
    FileTypePredicate {
        ft: FileType::File,
        follow: false,
    }
}

/// Creates a new `Predicate` that ensures the path points to a directory.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::path::is_dir();
/// assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
/// assert_eq!(true, predicate_fn.eval(Path::new("src")));
/// assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
/// ```
pub fn is_dir() -> FileTypePredicate {
    FileTypePredicate {
        ft: FileType::Dir,
        follow: false,
    }
}

/// Creates a new `Predicate` that ensures the path points to a symlink.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::path::is_symlink();
/// assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
/// assert_eq!(false, predicate_fn.eval(Path::new("src")));
/// assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
/// ```
pub fn is_symlink() -> FileTypePredicate {
    FileTypePredicate {
        ft: FileType::Symlink,
        follow: false,
    }
}
