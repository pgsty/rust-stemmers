# Changelog

## 1.2.1 — 2026-11-04

First tagged PGSTY maintenance release, based on TensorChord's
`51696378e352688b7ffd4fface615370ff5e8768` revision already used by `pg_tokenizer`.

- Add `Clone` for `Stemmer`, addressing original upstream PR #7.
- Add regression tests for multilingual stemming, all 30 algorithms' empty
  and Unicode input handling, ownership, and cloned stemmers across threads.
- Add Linux, macOS, and Windows CI for debug, release, documentation, and formatting.
- Point repository metadata and Git dependency examples at `pgsty/rust-stemmers`.
- Document all 30 algorithms and the incomplete historical Snowball sources.
- Rename the default branch to `main`.

The Snowball runtime and generated algorithms are unchanged from the base
revision. The crate still has no external Cargo dependencies. This is a Git
tag release, not a crates.io publication. See [UPSTREAM.md](UPSTREAM.md) for
the upstream issue review and follow-up work.
