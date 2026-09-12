// devela/build/main/linking/esp32_c3.rs
//
//! ESP32-C3 linker support.
//

use super::super::Build;

const DIRECT_BOOT: &[u8] = include_bytes!("../../../src/sys/hw/mcu/esp32/c3/direct_boot.x");
const FILE: &str = "esp32_c3_direct_boot.x";
const TARGET: &str = "riscv32imc-unknown-none-elf";

pub(super) fn main() -> Result<(), Box<dyn core::error::Error>> {
    Build::rerun_if_changed("src/sys/hw/mcu/esp32/c3/direct_boot.x");

    if std::env::var("TARGET").as_deref() != Ok(TARGET) {
        return Ok(());
    }

    #[cfg(feature = "__dbg")]
    Build::println(format!("- {FILE}"));

    let out = Build::out_dir();
    std::fs::write(out.join(FILE), DIRECT_BOOT)?;

    Build::emit_link_search(out.display());

    Ok(())
}
