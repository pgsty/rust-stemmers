# Historical Snowball sources

These `.sbl` files were inherited from
[CurrySoftware/rust-stemmers](https://github.com/CurrySoftware/rust-stemmers)
and are covered by the [BSD-3-Clause license](LICENSE).

This directory has 19 algorithm sources, while the Rust API exposes 30
algorithms. TensorChord's `51696378e352688b7ffd4fface615370ff5e8768` commit
updated the Rust output and runtime without synchronizing this directory.
The Rust output identifies its compiler as Snowball 2.2.0, but the exact
generator source commit and regeneration command were not recorded.

The shipped implementation is `src/snowball/algorithms/`. Do not assume these
historical sources reproduce it. A future regeneration should pin a released
[Snowball compiler](https://github.com/snowballstem/snowball), import its matching
runtime and algorithm sources together, and compare results against a pinned
reference corpus. Record all source revisions and commands in that change.
