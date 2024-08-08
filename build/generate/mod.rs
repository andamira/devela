// devela build::generate
//
//! Code generation during the build process.
//

#[cfg(feature = "_tuple")]
mod tuple;

pub(super) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "_tuple")]
    tuple::generate()?;

    Ok(())
}
