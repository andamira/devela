// devela construct::gen
//
//! Code generation during the build process.
//

#[cfg(feature = "_tuple")]
mod tuple;

pub(super) fn generate() -> Result<(), std::io::Error> {

    #[cfg(feature = "_tuple")]
    tuple::generate()?;

    Ok(())
}
