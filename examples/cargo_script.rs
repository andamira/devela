#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
devela = { path = ".." }
---
//
// devela::examples::cargo_script
//
//! A minimal standalone rust script.
//!
//! # Links
//! - WAIT:[RFC: cargo-script](https://github.com/rust-lang/rfcs/pull/3502)
//! - WAIT:[Tracking Issue for #3424](https://github.com/rust-lang/cargo/issues/12207)
//! - WAIT:[RFC: Syntax for embedding cargo-script manifests](https://github.com/rust-lang/rfcs/pull/3503)
//!     - [frontmatter syntax](https://github.com/rust-lang/rfcs/blob/ce78d/text/3503-frontmatter.md)
//
// These are multiple undocumented syntaxes for embedding the manifest ATM:
// - [Backticks](https://github.com/rust-lang/cargo/pull/13241):
// ```
// [dependencies]
// ```
// - [dashes](https://github.com/rust-lang/cargo/pull/13241):
// ---
// [dependencies]
// ---
// - [hashes](https://github.com/rust-lang/cargo/pull/13241):
// ###
// [dependencies]
// ###
// - [line-prefixes](https://github.com/rust-lang/cargo/pull/13247)
// ## [dependencies]

use devela::cdbg;

fn main() {
    cdbg!["hi"];
}
