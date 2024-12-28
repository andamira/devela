#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
devela = { path = "../.." }
---
// devela::examples::code::cargo_script
//
//! A minimal standalone rust script.
//
// WAIT:[Tracking Issue](https://github.com/rust-lang/cargo/issues/12207)

use devela::mod_from;

// A way to have modules in scripts
mod_from![shared, "../_shared.rs"];

fn main() {
    println!["{:?}", shared::HelloWorld];
}
