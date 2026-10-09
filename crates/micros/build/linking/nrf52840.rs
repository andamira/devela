//
//! nRF52840 linker support.
//

use super::super::Build;

const STANDALONE: &[u8] = include_bytes!("../../src/mcu/nrf/nrf52840/nrf52840.x");
const UF2_S140_V6: &[u8] = include_bytes!("../../src/mcu/nrf/nrf52840/nrf52840_uf2_s140_v6.x");
const STANDALONE_FILE: &str = "nrf52840.x";
const UF2_FILE: &str = "nrf52840_uf2_s140_v6.x";
const TARGET: &str = "thumbv7em-none-eabihf";

pub(super) fn main() -> Result<(), Box<dyn core::error::Error>> {
    Build::rerun_if_changed("src/mcu/nrf/nrf52840/nrf52840.x");
    Build::rerun_if_changed("src/mcu/nrf/nrf52840/nrf52840_uf2_s140_v6.x");

    if std::env::var("TARGET").as_deref() != Ok(TARGET) {
        return Ok(());
    }

    #[cfg(feature = "__dbg")]
    Build::println(format!("- {STANDALONE_FILE}, {UF2_FILE}"));

    let out = Build::out_dir();
    std::fs::write(out.join(STANDALONE_FILE), STANDALONE)?;
    std::fs::write(out.join(UF2_FILE), UF2_S140_V6)?;
    Build::emit_link_search(out.display());
    Ok(())
}
