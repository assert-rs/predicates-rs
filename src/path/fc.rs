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

use reflection;
use Predicate;

fn read_file(path: &path::Path) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    fs::File::open(path)?.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Predicate adapter that converts a `path` predicate to a byte predicate on its content.
///
/// This is created by `pred.from_path()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let buffer = read_file(path)?;
        Ok(self.p.eval(&buffer))
    }
}

impl<P> reflection::PredicateReflection for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn children<'a>(&'a self) -> Box<Iterator<Item = reflection::Child<'a>> + 'a> {
        let params = vec![reflection::Child::new("predicate", &self.p)];
        Box::new(params.into_iter())
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
