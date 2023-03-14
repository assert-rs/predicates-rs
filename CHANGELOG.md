# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Compatibility

- `color` feature no longer conditionally applies color.  Instead color will be applied when formatting as `{:#}`
- `color-auto` feature was removed
- Update MSRV to 1.64.0

## [2.1.5] - 2022-12-29

### Gixes

- Further generalized borrowing of predicates with `Borrow` trait

## [2.1.4] - 2022-12-02

## [2.1.3] - 2022-11-13

## [2.1.2] - 2022-11-05

### Compatibility

Update MSRV to 1.60

### Fixes

- Hide internal-only optional dependencies

## [2.1.1] - 2022-01-11

## [2.1.0] - 2021-11-22

### Fixed

- Report user's value (the actual)
  - Having the caller own this means you won't get intermediates like file paths
  - We already show it for the `diff` predicate
  - Now we show it in each leaf predicate (like Eq) and when adapting it (like taking a path and loading its file)
- Remove redundant result with `eq_file` assertions
- Clarify that the constant in Eq and Ord predicates is the expected value

## [2.0.3] - 2021-10-07

## [2.0.2] - 2021-08-16

### Added

- All predicates now implement `Send` and `Sync` when it's appropriate

## [2.0.1] - 2021-07-26

### Changed

- Upgraded `float-cmp`

## [2.0.0] - 2021-07-03

#### Breaking Changes

- `predicates::str::diff` was removed
- `predicates::str::similar` was renamed to `diff`
- The `difference` feature flag was renamed to `diff`
- `diff().split` and `diff().distance` were removed

#### Fixes

- Shrink the output of Diffs because its redundant
- Moved off of an unmaintained Diff library

## [1.0.8] - 2021-04-28

## [1.0.7] - 2021-01-29

## [1.0.6] - 2020-12-28

### Fixed
- `NamePredicate` now adds itself to the `Case` returned by `find_case`.

## [1.0.5] - 2020-07-18

### Fixed

- Update float-cmp dependency

## [1.0.4] - 2020-03-04
### Fixed

- Upgrade normalize-line-endings

## [1.0.3] - 2020-02-25

## [1.0.2] - 2019-11-18
### Fixed
- `BooleanPredicate` now implements `Predicate<T>` where `T: ?Sized`
  ([#84](https://github.com/assert-rs/predicates-rs/pull/84))

## [1.0.1] - 2019-04-22
### Changed
- BooleanPredicate is no longer generic, and is always Send and Sync.

## [1.0.0] - 2018-10-06

## [0.9.1] - 2018-10-05
### Added
- Created a predicate selection guide.

## [0.9.0] - 2018-07-30
### Added
- Support `?Sized` types for `FnPredicate`.
- Add `str_pred.normalize()` predicate.
- Add reflection to `Predicate`.
- Add support for predicates returning why they failed (`find_case`) which can
  be combined with the new `predicates-tree` crate.
- Split out `predicates-core` for reducing ecosystem breaking changes.

### Changed
- Predicates must also implement `PredicateReflection`

## [0.5.2] - 2018-07-20
### Added
- **path:**  support file-based str predicates ([4b430532](https://github.com/assert-rs/predicates-rs/commit/4b430532f7cd660bd813863871ede6f108e7be67), closes [#56](https://github.com/assert-rs/predicates-rs/issues/56))
-   Expand trait coverage ([33972a7d](https://github.com/assert-rs/predicates-rs/commit/33972a7d0c92eb7f7c7e95af4bb35bea0ac810ab))

## [0.5.1] - 2018-06-05
### Added
-   Fix eq for str ([7650e9e6](https://github.com/assert-rs/predicates-rs/commit/7650e9e6d43f2ddd047ad8defa0c724b31ebd1c4))

## [0.5.0] - 2018-05-30
### Added
- **trait:**
  -  Allow naming `Predicate` expressions
- **str:**
  -  Add regex repetition count, closes #27
  -  from_utf8 adapter, closes #21
  -  Trimming predicate decorator
- **path:**
  - `eq_file` predicate to test a file-under-test with a fixture, closes #32.
    - `eq_file(...).utf()` adapter to do string comparisons with the fixture
  - Add a `from_file_path` extension method to `Predicate<[u8]>` that turns it into a `Predicate<Path>`, closes #31.

### Breaking Changes
- **trait:**
  -  All `Predicate`s are now `Display` ([05216708](https://github.com/assert-rs/predicates-rs/commit/05216708359544f2c5f3a256f50c012f521c39a6), breaks [#](https://github.com/assert-rs/predicates-rs/issues/))
  -  Decouple boxing from trait ([f981fac3](https://github.com/assert-rs/predicates-rs/commit/f981fac39271746162365f3c577cffac730e1d97), breaks [#](https://github.com/assert-rs/predicates-rs/issues/))
  -  Decouple boolean logic from trait ([88b72f9e](https://github.com/assert-rs/predicates-rs/commit/88b72f9ef58a86f2af68c0510d99326f5e644f76), breaks [#](https://github.com/assert-rs/predicates-rs/issues/))

## [0.4.0] - 2018-05-10
### Added
- Define oldest supported version of Rust as 1.22.
- CI that ensures
  - works on Windows and Linux
  - works on 1.22 to nightly
- **float:** `is_close` Predicate (see #11).
- **path:**
  -  File type predicates: `is_file`, `is_dir`, `is_symlink` (see #8).
  -  Existence predicate: `exists`, `missing` (see #8).
- **str:**
  -  Basic string predicates: `is_empty`, `starts_with`, `ends_with`, and `contains` with optional count (see #25).
  -  Regex predicate (see #12).
  -  Edit-distance predicate (see #9).

### Changed
- Clearly delineate API from prelude (see #17).
- Switch `Predicate` trait from Associated Types to Generics.
- **iter:**
  -  Renamed `set` predicates as `iter` predicates to clarify the intent from some implementation.
  -  Remove ambiguity of predicate factories (see #24):
    - `contains` -> `in_iter`
    - `contains_hashable` -> `in_hash`
  - Turned `contains_ord` into a specialization of `in_iter` by adding a `sort` method.

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

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v2.1.5...HEAD
[2.1.5]: https://github.com/assert-rs/predicates-rs/compare/v2.1.4...v2.1.5
[2.1.4]: https://github.com/assert-rs/predicates-rs/compare/v2.1.3...v2.1.4
[2.1.3]: https://github.com/assert-rs/predicates-rs/compare/v2.1.2...v2.1.3
[2.1.2]: https://github.com/assert-rs/predicates-rs/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/assert-rs/predicates-rs/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/assert-rs/predicates-rs/compare/v2.0.3...v2.1.0
[2.0.3]: https://github.com/assert-rs/predicates-rs/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/assert-rs/predicates-rs/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/assert-rs/predicates-rs/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/assert-rs/predicates-rs/compare/v1.0.8...v2.0.0
[1.0.8]: https://github.com/assert-rs/predicates-rs/compare/v1.0.7...v1.0.8
[1.0.7]: https://github.com/assert-rs/predicates-rs/compare/v1.0.6...v1.0.7
[1.0.6]: https://github.com/assert-rs/predicates-rs/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/assert-rs/predicates-rs/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/assert-rs/predicates-rs/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/assert-rs/predicates-rs/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/assert-rs/predicates-rs/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/assert-rs/predicates-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/assert-rs/predicates-rs/compare/v0.9.1...v1.0.0
[0.9.1]: https://github.com/assert-rs/predicates-rs/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/assert-rs/predicates-rs/compare/v0.5.2...v0.9.0
[0.5.2]: https://github.com/assert-rs/predicates-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/assert-rs/predicates-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/assert-rs/predicates-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/assert-rs/predicates-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/assert-rs/predicates-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/assert-rs/predicates-rs/compare/v0.1.0...v0.2.0
