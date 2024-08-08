// devela build
//
//! The build script.
//

mod features;
mod generate;
mod utils;

fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    utils::println(&format!("OUT_DIR: {}", out_dir()));

    features::main()?;
    generate::main()?;

    Ok(())
}
