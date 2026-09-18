//
//! Build-time configuration and linking support.
//!
//
// lints
#![cfg_attr(test, allow(dead_code))]
#![allow(unexpected_cfgs, reason = "_build_namespace std feature-gate")]
//
// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg))]

/* helpers */

#[allow(unused)]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

/* globals */

#[allow(unused)]
const CRATE_NAME: &str = "devela_micros";

/* imports */

extern crate self as build;

// NOTE: manually imports the Build namespace from devela
items! {
    macro_rules! _TAG_NAMESPACE {()=>{""}} #[allow(unused)] use _TAG_NAMESPACE;
    #[allow(unused)] mod _build_namespace; // SYMLINK TO ../../devela/src/code/build/namespace.rs
    #[allow(unused_imports)] pub(crate) use _build_namespace::Build;
}

/* build modules */

mod environment; // SYMLINK to ../../devela/build/environment.rs
mod linking; // target linker support

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    Build::println_start_end("devela_micros build script", true);

    environment::main()?;
    linking::main()?;

    #[cfg(feature = "__dbg")]
    Build::println_start_end("devela_micros build script", false);
    Ok(())
}
