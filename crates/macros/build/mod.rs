// devela_macros::build
//
//! Build-time code generation and configuration.
//
// lints
#![allow(unexpected_cfgs)]
//
// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]

/* helpers */

#[cfg(feature = "__dbg")]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

/* globals */

#[allow(unused)]
const CRATE_NAME: &str = "devela_macros";

/* imports */

// NOTE: manually imports the Build namespace from devela_base_std
#[cfg(feature = "__dbg")]
items! {
    macro_rules! _TAG_NAMESPACE {()=>{""}} use _TAG_NAMESPACE;
    #[allow(unused)]
    mod _imports_std; // SYMLINK TO /crates/base/std/src/build/namespace.rs
    #[allow(unused_imports)] use _imports_std::Build;
}

/* build modules */

mod alias; // SYMLINK to /devela/build/main/alias.rs
mod environment; // SYMLINK to /devela/build/main/environment.rs
mod features; // SYMLINK to /devela/build/main/features.rs

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    Build::println_start_end("macros build script", true);

    alias::main()?;

    environment::main()?;
    features::main()?;

    #[cfg(feature = "__dbg")]
    Build::println_start_end("macros build script", false);
    Ok(())
}
