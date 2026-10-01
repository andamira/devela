//
//! ESP32-S3 linker support.
//

use super::super::Build;

const MEMORY: &[u8] = include_bytes!("../../src/mcu/esp32/s3/memory.x");
const ROM: &[u8] = include_bytes!("../../src/mcu/esp32/s3/rom.x");

const MEMORY_FILE: &str = "memory.x";
const ROM_FILE: &str = "rom.x";
const TARGET: &str = "xtensa-esp32s3-none-elf";

pub(super) fn main() -> Result<(), Box<dyn core::error::Error>> {
    Build::rerun_if_changed("src/mcu/esp32/s3/memory.x");
    Build::rerun_if_changed("src/mcu/esp32/s3/rom.x");

    if std::env::var("TARGET").as_deref() != Ok(TARGET) {
        return Ok(());
    }

    #[cfg(feature = "__dbg")]
    {
        Build::println(format!("- {MEMORY_FILE}"));
        Build::println(format!("- {ROM_FILE}"));
    }

    let out = Build::out_dir();

    std::fs::write(out.join(MEMORY_FILE), MEMORY)?;
    std::fs::write(out.join(ROM_FILE), ROM)?;

    Build::emit_link_search(out.display());

    Ok(())
}
