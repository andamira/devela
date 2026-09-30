//
//! ZX Spectrum 48K linker support.
//

use super::super::Build;

const LINKER_SCRIPT: &[u8] = include_bytes!("../../src/computer/zx/spectrum/s48/spectrum48.x");
const FILE: &str = "spectrum48.x";
const TARGET: &str = "z80-unknown-none-elf";

pub(super) fn main() -> Result<(), Box<dyn core::error::Error>> {
    Build::rerun_if_changed("src/computer/zx/spectrum/s48/spectrum48.x");

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
