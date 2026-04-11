#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
devela = { path = "../..", features = [] }
---
// devela::examples::code::cargo_script
//
//! A minimal standalone rust script.
//
// WAIT:[cargo-script](https://github.com/rust-lang/cargo/pull/16569)

use devela::{cdbg, mod_from};

mod_from![shared, "../_shared.rs"]; // load modules from scripts!
use shared::HelloWorld;

fn main() {
    cdbg![HelloWorld];
}
