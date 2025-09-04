// devela::build
//
//! Build-time code generation and configuration.
//!
#![doc = include_str!("./Mod.md")]
//
// NOTE: imported from `../src/_info/mod.rs`.
// NOTE: Have to use relative imports (super::*) instead of crate::*,
// so that it can also work when compiling the private documentation.
//
// SEE:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// WAIT: [reduce need for build scrips](https://github.com/rust-lang/cargo/issues/14948)
// WAIT: [sandbox/jail build scripts](https://github.com/rust-lang/cargo/issues/5720)
// WAIT: [post-build execution](https://github.com/rust-lang/cargo/issues/545)
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(test, allow(dead_code))]

/* globals */

#[allow(unused)]
const CRATE_NAME: &str = "devela";

/* imports */

extern crate devela_base as base;
#[cfg(feature = "__build")]
extern crate devela_base_std as base_std;
extern crate self as build;

#[allow(unused)]
#[cfg(feature = "__build")]
use base_std::Build;

/* build modules */

mod environment;
mod features;

#[cfg(feature = "__build")]
mod codegen; // tuple, unroll

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    Build::println_start_end("main build script", true);

    environment::main()?;
    features::main()?;

    #[cfg(feature = "__build")]
    codegen::main()?;

    #[cfg(feature = "__dbg")]
    Build::println_start_end("main build script", false);
    Ok(())
}
