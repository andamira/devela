// devela_postbuild::build
//
//! Post-build-time orchestrator.
//

/* imports */

extern crate devela_base as base;
extern crate devela_base_std as base_std;

#[allow(unused)]
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
