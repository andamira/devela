// devela::meta::build
//
//! Build-time code generation and configuration.
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

mod codegen; // tuple, unroll
mod environment;
mod features;
mod utils; // out_dir, manifest_dir, manifest_path, println* , TAB*

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
    codegen::main()?;

    #[cfg(feature = "__dbg")]
    utils::println_start_end(false);
    Ok(())
}
