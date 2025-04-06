#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
devela = { path = "../..", features = [] }
---
// devela::examples::code::cargo_script
//
//! A minimal standalone rust script.
//
// WAIT:[Tracking Issue](https://github.com/rust-lang/cargo/issues/12207)
// NOTE: doesn't work inside a workspace:
// - https://github.com/rust-lang/cargo/issues/12207#issuecomment-2037176136

use devela::{cdbg, mod_from};

mod_from![shared, "../_shared.rs"]; // load modules from scripts!
use shared::HelloWorld;

fn main() {
    cdbg![HelloWorld];
}
