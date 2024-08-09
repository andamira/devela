// devela build
//
//! The build script.
//

mod environment;
mod features;
mod generate;
mod utils;

fn main() -> Result<(), std::io::Error> {
    environment::main()?;
    features::main()?;
    generate::main()?;

    Ok(())
}
