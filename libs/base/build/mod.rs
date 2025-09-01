// devela_base::build
//
//! Build-time code generation and configuration.
//

/* helpers */

#[cfg(feature = "__dbg")]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

/* manual includes */

// NOTE: the Build namespace from devela_base_std for the environment script
#[cfg(feature = "__dbg")]
items! {
    macro_rules! TAG_NAMESPACE {()=>{""}} use TAG_NAMESPACE;
    #[path = "../../base_std/src/build/namespace.rs"] #[allow(unused)]
    mod imports; use imports::Build;
}

/* build modules */

mod environment; // NOTE: a symlink to /devela/build/main/environment.rs
mod features;

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(true);

    environment::main()?;
    features::main()?;

    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(false);
    Ok(())
}
