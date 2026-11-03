# Rust Stemmers

Pure Rust implementations of 30 stemming algorithms from the
[Snowball project](https://snowballstem.org/), with no Cargo dependencies.

This is the [PGSTY maintenance fork](https://github.com/pgsty/rust-stemmers),
used by [pg_tokenizer](https://github.com/pgsty/pg_tokenizer). It descends from
[CurrySoftware/rust-stemmers](https://github.com/CurrySoftware/rust-stemmers)
through TensorChord's `51696378e352688b7ffd4fface615370ff5e8768` update.
The generated stemmers identify their compiler as Snowball 2.2.0.

Release `v1.2.1` preserves that revision's algorithms and runtime, adds
`Clone` for `Stemmer`, and restores automated regression testing. See the
[changelog](CHANGELOG.md) and [upstream review](UPSTREAM.md) for scope and
known limitations. The default branch is `main`.

# Supported Algorithms

-   Arabic
-   Armenian
-   Basque
-   Catalan
-   Danish
-   Dutch
-   English
-   Estonian
-   Finnish
-   French
-   German
-   Greek
-   Hindi
-   Hungarian
-   Indonesian
-   Irish
-   Italian
-   Lithuanian
-   Nepali
-   Norwegian
-   Porter (original English Porter algorithm)
-   Portuguese
-   Romanian
-   Russian
-   Serbian
-   Spanish
-   Swedish
-   Tamil
-   Turkish
-   Yiddish


# Usage

Pin this fork by tag in `Cargo.toml`:

```toml
[dependencies]
rust-stemmers = { git = "https://github.com/pgsty/rust-stemmers.git", tag = "v1.2.1" }
```

Keep the application's `Cargo.lock` to pin the resolved commit as well.
This tag is a Git dependency release; the crates.io package is maintained
separately by the original upstream.

```rust
extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

// Create a stemmer for the English language (Porter2)
let en_stemmer = Stemmer::create(Algorithm::English);

// Stem the word "fruitlessly"
// Please be aware that all algorithms expect their input to only contain lowercase characters.
assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
```

# Development

```sh
cargo fmt --check
cargo test --offline
cargo test --offline --release
```

Tests cover every supported algorithm's handling of empty and Unicode input,
multilingual stemming examples, English Porter/Porter2 differences, and
the ownership behavior of the public API. GitHub Actions runs on Linux,
macOS, and Windows.

The checked-in Rust files are the shipped implementation. No Snowball compiler
is needed to build this crate. The historical `.sbl` files in `algorithms/`
are incomplete; see [their README](algorithms/README.md) before regenerating.

## License

The original Rust wrapper is covered by the [MIT license](LICENSE); Snowball
algorithms carry their [BSD-3-Clause license](algorithms/LICENSE).

# Related Projects

-   The [stemmer](https://github.com/lise-henry/stemmer-rs) crate provides bindings to the C Snowball implementation.

