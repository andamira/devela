// devela build::generate
//
//! Code generation during the build process.
//

#[cfg(feature = "_tuple")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_tuple")))]
mod tuple;

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    super::utils::println_heading("Code generation:");

    #[cfg(feature = "_tuple")]
    tuple::generate()?;

    Ok(())
}
