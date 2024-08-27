// devela build
//
//! The build script.
//

mod environment;
mod features;
mod generate;
mod utils;

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

// WAIT:1.81 [error_in_core](https://github.com/rust-lang/rust/pull/125951)
fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "__dbg")]
    utils::println_start_end(true);

    environment::main()?;
    features::main()?;
    generate::main()?;

    #[cfg(feature = "__dbg")]
    utils::println_start_end(false);
    Ok(())
}
