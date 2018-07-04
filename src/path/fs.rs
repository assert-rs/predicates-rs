// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path;
use std::str;

use Predicate;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileContent(Vec<u8>);

impl FileContent {
    pub fn new(path: &path::Path) -> io::Result<FileContent> {
        let mut buffer = Vec::new();
        fs::File::open(path)?.read_to_end(&mut buffer)?;
        Ok(FileContent(buffer))
    }

    pub fn utf8(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.0)
    }
}

/// Predicate that compares file matches
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryFilePredicate {
    path: path::PathBuf,
    file_content: FileContent,
}

impl BinaryFilePredicate {
    fn eval(&self, path: &path::Path) -> io::Result<bool> {
        let content = FileContent::new(path)?;
        Ok(self.file_content == content)
    }

    /// Creates a new `Predicate` that ensures complete equality
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use predicates::prelude::*;
    ///
    /// let predicate_file = predicate::path::eq_file(Path::new("Cargo.toml")).utf8().unwrap();
    /// assert_eq!(true, predicate_file.eval(Path::new("Cargo.toml")));
    /// assert_eq!(false, predicate_file.eval(Path::new("Cargo.lock")));
    /// assert_eq!(false, predicate_file.eval(Path::new("src")));
    /// ```
    pub fn utf8(self) -> Option<StrFilePredicate> {
        let path = self.path;
        self.file_content.utf8().ok().map(|s| StrFilePredicate {
            path,
            content: s.to_string(),
        })
    }
}

impl Predicate<path::Path> for BinaryFilePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }
}

impl fmt::Display for BinaryFilePredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var is {}", self.path.display())
    }
}

/// Creates a new `Predicate` that ensures complete equality
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::prelude::*;
///
/// let predicate_file = predicate::path::eq_file(Path::new("Cargo.toml"));
/// assert_eq!(true, predicate_file.eval(Path::new("Cargo.toml")));
/// assert_eq!(false, predicate_file.eval(Path::new("src")));
/// assert_eq!(false, predicate_file.eval(Path::new("Cargo.lock")));
/// ```
pub fn eq_file(path: &path::Path) -> BinaryFilePredicate {
    let file_content = FileContent::new(path).unwrap();
    BinaryFilePredicate {
        path: path.to_path_buf(),
        file_content,
    }
}

/// Predicate that compares string content of files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrFilePredicate {
    path: path::PathBuf,
    content: String,
}

impl StrFilePredicate {
    fn eval(&self, path: &path::Path) -> Option<bool> {
        let content = FileContent::new(path).ok()?;
        let string = content.utf8().ok()?;
        Some(self.content == string)
    }
}

impl fmt::Display for StrFilePredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var is {}", self.path.display())
    }
}

impl Predicate<path::Path> for StrFilePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }
}
