# predicates-rs

This library is an implementation of **boolean-valued predicate functions** in Rust.

[![Build Status](https://travis-ci.org/nastevens/predicates-rs.svg?branch=master)](https://travis-ci.org/nastevens/predicates-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/yl5w3ittk4kggfsh?svg=true)](https://ci.appveyor.com/project/nastevens/predicates-rs)
[![Crates.io](https://img.shields.io/crates/v/predicates.svg?maxAge=2592000)](https://crates.io/crates/predicates)

[Documentation](https://docs.rs/predicates)


## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
predicates = "0.2"
```

Next, add this to your crate:

```rust
extern crate predicates;

use predicates::Predicate;
```

For more information on using predicates, look at the
[documentation](https://docs.rs/predicates)


# License

`predicates-rs` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.
