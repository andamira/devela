//
//! SAM3X8E linker support.
//

use super::super::Build;

const LINKER_SCRIPT: &[u8] = include_bytes!("../../src/mcu/sam/sam3x8e/sam3x8e.x");
const FILE: &str = "sam3x8e.x";
const TARGET: &str = "thumbv7m-none-eabi";

pub(super) fn main() -> Result<(), Box<dyn core::error::Error>> {
    Build::rerun_if_changed("src/mcu/sam/sam3x8e/sam3x8e.x");

    if std::env::var("TARGET").as_deref() != Ok(TARGET) {
        return Ok(());
    }

    #[cfg(feature = "__dbg")]
    Build::println(format!("- {FILE}"));

    let out = Build::out_dir();
    std::fs::write(out.join(FILE), LINKER_SCRIPT)?;

    Build::emit_link_search(out.display());

    Ok(())
}
