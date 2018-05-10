# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - Unreleased

## [0.4.0] - 2018-05-10
### Added
* Define oldest supported version of Rust as 1.22.
* CI that ensures
  * works on Windows and Linux
  * works on 1.22 to nightly
* **float:** `is_close` Predicate (see #11).
* **path:**
  *  File type predicates: `is_file`, `is_dir`, `is_symlink` (see #8).
  *  Existence predicate: `exists`, `missing` (see #8).
* **str:**
  *  Basic string predicates: `is_empty`, `starts_with`, `ends_with`, and `contains` with optional count (see #25).
  *  Regex predicate (see #12).
  *  Edit-distance predicate (see #9).

### Changed
* Clearly delineate API from prelude (see #17).
* Switch `Predicate` trait from Associated Types to Generics.
* **iter:**
  *  Renamed `set` predicates as `iter` predicates to clarify the intent from some implementation.
  *  Remove ambiguity of predicate factories (see #24):
    * `contains` -> `in_iter`
    * `contains_hashable` -> `in_hash`
  * Turned `contains_ord` into a specialization of `in_iter` by adding a `sort` method.

## [0.3.0] - 2017-06-26
### Added
- `BoxPredicate` type that wraps a `Predicate` trait object to make it easier
  to store and work with predicates through a program. Also implements `Debug`
  and `Display` wrappers as a convenience.
- `FnPredicate` type that wraps a function of the type `Fn(&T) -> bool` in a
  `Predicate` type.

### Changed
- The `boxed` function now returns a type `BoxPredicate` instead of a type
  alias.
- The `Item` type parameter of `Predicate` no longer has the `Sized`
  restriction.

## [0.2.0] - 2017-06-02
### Added
- This changelog

### Fixed
- Made modules under `predicate` private, with their public interfaces exposed
  through `pub use` in the `predicate` `mod.rs` file.

## 0.1.0 - 2017-06-02
### Added
- Initial commit of functional code
- Continuous integration with Travis (Linux) and AppVeyor (Windows)
- Basic README

[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/assert-rs/predicates-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/assert-rs/predicates-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/assert-rs/predicates-rs/compare/v0.1.0...v0.2.0
