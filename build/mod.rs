// devela build
//
//! The build script.
//

mod environment;
mod features;
mod generate;
mod utils;

fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    utils::println_start_end(true);

    environment::main()?;
    features::main()?;
    generate::main()?;

    #[cfg(feature = "__dbg")]
    utils::println_start_end(false);
    Ok(())
}
