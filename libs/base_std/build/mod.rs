// devela_base_std::build
//
//! Build-time code generation and configuration.
//

/* helpers */

#[cfg(feature = "__dbg")]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

/* globals */

#[allow(unused)]
const CRATE_NAME: &str = "devela_base_std";

/* imports */

// NOTE: manually imports the Build namespace from devela_base_std
#[cfg(feature = "__dbg")]
items! {
    macro_rules! _TAG_NAMESPACE {()=>{""}} use _TAG_NAMESPACE;
    #[path = "../../base_std/src/build/namespace.rs"] #[allow(unused)]
    mod imports; use imports::Build;
}

/* build modules */

mod alias; // NOTE: symlink to /devela/build/main/alias.rs
mod environment; // NOTE: symlink to /devela/build/main/environment.rs
mod features; // NOTE: symlink to /devela/build/main/features.rs

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    Build::println_start_end("base_std build script", true);

    alias::main()?;

    environment::main()?;
    features::main()?;

    #[cfg(feature = "__dbg")]
    Build::println_start_end("base_std build script", false);
    Ok(())
}
