// devela build::generate
//
//! Code generation during the build process.
//

#[cfg(feature = "_tuple")]
mod tuple;

pub(super) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    crate::utils::println_heading("Code generation:");

    #[cfg(feature = "_tuple")]
    tuple::generate()?;

    Ok(())
}
