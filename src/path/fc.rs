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

#[derive(Clone, Debug, PartialEq)]
pub struct FileContent(Vec<u8>);

impl FileContent {
    pub fn new(path: &path::Path) -> io::Result<FileContent> {
        let mut buffer = Vec::new();
        fs::File::open(path)?.read_to_end(&mut buffer)?;
        Ok(FileContent(buffer))
    }
    pub fn utf8(&self) -> Result<String, str::Utf8Error> {
        str::from_utf8(&self.0).map(|s| s.to_string())
    }
}

/// Predicate adaper that converts a `path` to file content predicate to byte predicate.
///
/// This is created by `pred.from_path()`.
#[derive(Clone, Debug)]
pub struct FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    p: P,
}

impl<P> FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn eval(&self, path: &path::Path) -> io::Result<bool> {
        let buffer = FileContent::new(path)?;
        Ok(self.p.eval(&buffer.0))
    }
}

impl<P> fmt::Display for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.p)
    }
}

impl<P> Predicate<path::Path> for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }
}

/// `Predicate` extension adapting a `slice` Predicate.
pub trait PredicateFileContentExt
where
    Self: Predicate<[u8]>,
    Self: Sized,
{
    /// Returns a `FileContentPredicate` that adapts `Self` to a file content `Predicate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use std::path::Path;
    ///
    /// let predicate_fn = predicate::str::is_empty().not().from_utf8().from_file_path();
    /// assert_eq!(true, predicate_fn.eval(Path::new("./tests/hello_world")));
    /// assert_eq!(false, predicate_fn.eval(Path::new("./tests/empty_file")));
    /// ```
    fn from_file_path(self) -> FileContentPredicate<Self> {
        FileContentPredicate { p: self }
    }
}

impl<P> PredicateFileContentExt for P
where
    P: Predicate<[u8]>,
{
}
