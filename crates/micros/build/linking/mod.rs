// build/main/linking/mod.rs
//
//! Link-time support.
//

mod esp32_c3;

pub(crate) fn main() -> Result<(), Box<dyn core::error::Error>> {
    #[cfg(feature = "__dbg")]
    super::Build::println_heading("Linking support:");

    esp32_c3::main()?;

    Ok(())
}
