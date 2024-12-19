#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
devela = { path = ".." }
---
// devela::examples::cargo_script
//
//! A minimal standalone rust script.
//
// WAIT:[Tracking Issue](https://github.com/rust-lang/cargo/issues/12207)

use devela::cdbg;

fn main() {
    cdbg!["hi"];
}
