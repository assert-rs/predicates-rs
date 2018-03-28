# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - Unreleased

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

[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/assert-rs/predicates-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/assert-rs/predicates-rs/compare/v0.1.0...v0.2.0
