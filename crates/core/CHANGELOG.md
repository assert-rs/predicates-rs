# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Compatibility

- Update MSRV to 1.70.0

## [1.0.6] - 2023-03-14

### Compatibility

- Update MSRV to 1.64.0

## [1.0.5]

### Compatibility

Update MSRV to 1.60

## [0.9.0] - 2018-07-30

### Added
- Add reflection to `Predicate`.
- Add support for predicates returning why they failed (`find_case`) which can
  be combined with the new `predicates-tree` crate.
- Split out `predicates-core` for reducing ecosystem breaking changes.

### Changed
- Predicates must also implement `PredicateReflection`

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/predicates-core-v1.0.6...HEAD
[1.0.6]: https://github.com/assert-rs/predicates-rs/compare/v0.9.0...predicates-core-v1.0.6
[0.9.0]: https://github.com/assert-rs/predicates-rs/compare/v0.5.2...v0.9.0
