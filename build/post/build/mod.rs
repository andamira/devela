// devela_postbuild::build
//
//! Post-build-time orchestrator.
//

/* imports */

extern crate devela_base as base;

/* modules */

// mod postbuild;

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(true);

    // postbuild::main()?;

    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(false);
    Ok(())
}
