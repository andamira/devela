// devela_base_std::build
//
//! Build-time code generation and configuration.
//

mod environment;
// mod features;

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

    // #[cfg(feature = "__dbg")]
    // utils::println_start_end(false);
    Ok(())
}
