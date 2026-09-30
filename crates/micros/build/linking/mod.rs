// build/main/linking/mod.rs
//
//! Link-time support.
//

mod esp32_c3;
mod esp32_c6;
mod sam3x8e;
#[cfg(feature = "spectrum48")]
mod spectrum48;

pub(crate) fn main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    super::Build::println_heading("Linking support:");

    esp32_c3::main()?;
    esp32_c6::main()?;
    sam3x8e::main()?;
    #[cfg(feature = "spectrum48")]
    spectrum48::main()?;

    Ok(())
}
