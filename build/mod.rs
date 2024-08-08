// devela build
//
//! The build script.
//

mod features;
mod generate;

fn main() -> Result<(), std::io::Error> {
    features::main()?;
    generate::main()?;

    Ok(())
}
