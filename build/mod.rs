// devela::build
//
//!
//
// NOTE: imported from `../src/_info/mod.rs`.
// NOTE: Have to use relative imports (super::*) instead of crate::*,
// so that it can also work when compiling the private documentation.
//
// SEE:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(test, allow(dead_code))]

mod environment;
mod features;
mod generate;
mod utils;
pub(crate) use utils::*;

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    utils::println_start_end(true);

    environment::main()?;
    features::main()?;
    generate::main()?;

    #[cfg(feature = "__dbg")]
    utils::println_start_end(false);
    Ok(())
}
