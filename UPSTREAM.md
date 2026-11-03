# Upstream review — 2026-11-04

Reviewed [CurrySoftware/rust-stemmers](https://github.com/CurrySoftware/rust-stemmers)
at `1fde09dfa08fb0fb07fbd6570aba8ecc98174edb`, its four open issues and five
open pull requests, and the inherited TensorChord commit
`51696378e352688b7ffd4fface615370ff5e8768`.

## Findings and disposition

| Upstream item | Status in this fork |
| --- | --- |
| [PR #7: Clone for Stemmer](https://github.com/CurrySoftware/rust-stemmers/pull/7) | Implemented in v1.2.1, with a test using a cloned stemmer on another thread. |
| [PR #20: Armenian type mismatch](https://github.com/CurrySoftware/rust-stemmers/pull/20) | Already addressed by the inherited runtime/algorithm update. Runtime cursors and Armenian context fields use signed integers. The Snowball maintainer recommends matching compiler and runtime versions instead of changing generated fields to unsigned integers. |
| [PR #19: optional serde](https://github.com/CurrySoftware/rust-stemmers/pull/19), [PR #24: remove serde](https://github.com/CurrySoftware/rust-stemmers/pull/24) | The inherited fork already removed serde and its derives. There are no Cargo dependencies. v1.2.1 preserves that API; it does not add a serde feature. |
| [Issue #21: internal / international / interning](https://github.com/CurrySoftware/rust-stemmers/issues/21) | Reproduces: all three reduce to `intern`. Recorded in a compatibility test. This is an algorithm behavior issue, not a compile failure; changing it needs corpus comparison and a migration plan for existing tokenized data. |
| [Issue #23](https://github.com/CurrySoftware/rust-stemmers/issues/23), [PR #22: Czech](https://github.com/CurrySoftware/rust-stemmers/pull/22) | Still missing. Add from a pinned Snowball release with its matching runtime and reference tests. The old PR mixes generated code with the earlier runtime and is not imported here. |
| [Issue #26: Polish](https://github.com/CurrySoftware/rust-stemmers/issues/26) | Still missing. Treat as part of the same versioned Snowball update. |
| [Issue #27: update Snowball](https://github.com/CurrySoftware/rust-stemmers/issues/27) | Still applicable: generated files identify Snowball 2.2.0. A future update should pin a stable release and its source commit, regenerate the algorithms and runtime together, and document output changes. The Snowball maintainer explicitly recommends released tags instead of tracking its development branch. |

## Additional local findings

- The inherited update removed the full test corpus and all unit tests, leaving
  only one doctest. v1.2.1 adds a small regression suite and CI. These checks do
  not replace exhaustive comparison with Snowball's language corpora.
- README listed only 18 of the 30 algorithms in the public API. The list is now
  complete, and dependency instructions use a pinned PGSTY tag.
- `algorithms/` contains 19 historical `.sbl` files and does not reproduce all
  shipped Rust output. Its README now records this gap; regeneration remains
  follow-up work.
- `cargo clippy --offline --all-targets -- -D warnings` fails on the inherited
  Snowball code with Rust/Clippy 1.99.1: 2,408 diagnostics per library build,
  chiefly generated single-pass loops, unnecessary casts, and redundant
  lifetimes. All reported source locations are under the unchanged
  `src/snowball/` directory. Strict Clippy is not a passing release check;
  generator-aware lint policy or regeneration is follow-up work.

## Follow-up release scope

Pin a released Snowball compiler and a reference-data commit. Import its matching
runtime and all algorithm sources, record a reproducible generation command,
then compare old and new results over the supported languages. Add Czech and
Polish with tests and update `pg_tokenizer`'s language mapping separately.
Document any required re-tokenization or index rebuild before consumers upgrade.

The v1.2.1 tag provides a maintained source location for the currently deployed
algorithm set. It does not claim to fix the unresolved language or stemming
behavior issues above, and no upstream issues or pull requests were modified.
