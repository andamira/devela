// devela_base::build
//
//! Build-time code generation and configuration.
//

// #![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(test, allow(dead_code))]

mod environment;
// mod features;

// MAYBE: import from src::code::util / build_util ? using manual mod… and use in devela build
// mod utils; // out_dir, manifest_dir, manifest_path, println* , TAB*

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> Result<(), Box<dyn core::error::Error>> {
    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(true);

    environment::main()?;
    // features::main()?;
    // codegen::main()?;

    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(false);
    Ok(())
}
