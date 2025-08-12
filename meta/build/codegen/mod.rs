// devela::meta::build::codegen
//
//! Code generation during the build process.
//

#[cfg(feature = "_tuple")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
mod tuple;
#[cfg(feature = "_unroll")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_unroll")))]
mod unroll;

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    super::utils::println_heading("Code generation:");

    #[cfg(feature = "_tuple")]
    tuple::generate()?;
    #[cfg(feature = "_unroll")]
    unroll::generate()?;

    Ok(())
}
