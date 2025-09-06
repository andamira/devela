// devela_postbuild::build
//
//! Post-build-time orchestrator.
//
// `$ cargo wc -F __dbg`
//

/* imports */

extern crate devela_base_core as base_core;
#[cfg(feature = "__build")]
extern crate devela_base_std as base_std;

#[allow(unused)]
#[cfg(feature = "__build")]
use base_std::Build;

/* modules */

// mod postbuild; // TODO

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    Build::println_start_end("post-build script", true);

    // postbuild::main()?;

    #[cfg(feature = "__dbg")]
    Build::println_start_end("post-build script", false);
    Ok(())
}
