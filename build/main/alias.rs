// devela::build::alias
//
//! Aliases of combinations of configuration features.
//

#[cfg(feature = "__dbg")]
use super::Build;

/// Defines a cfg flag `alias` and saves its name in a `list`.
fn new_alias(list: &mut Vec<&'static str>, alias: &'static str) {
    println!("cargo:rustc-cfg={}", alias);
    list.push(alias);
}

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    Build::println_heading("Aliases:");

    let mut aliases = Vec::with_capacity(20);

    /* arch */
    // 1

    #[cfg(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    ))]
    new_alias(&mut aliases, "any_supported_arch");

    /* base */
    // 14

    #[cfg(all(feature = "base_safe", feature = "safe"))]
    new_alias(&mut aliases, "base_safe");

    #[cfg(all(feature = "base_safe", feature = "safe_build"))]
    new_alias(&mut aliases, "base_safe_build");

    #[cfg(all(feature = "base_safe", feature = "safe_code"))]
    new_alias(&mut aliases, "base_safe_code");
    #[cfg(all(feature = "base_safe", feature = "safe_data"))]
    new_alias(&mut aliases, "base_safe_data");
    #[cfg(all(feature = "base_safe", feature = "safe_game"))]
    new_alias(&mut aliases, "base_safe_game");
    #[cfg(all(feature = "base_safe", feature = "safe_lang"))]
    new_alias(&mut aliases, "base_safe_lang");
    #[cfg(all(feature = "base_safe", feature = "safe_media"))]
    new_alias(&mut aliases, "base_safe_media");
    #[cfg(all(feature = "base_safe", feature = "safe_mem"))]
    new_alias(&mut aliases, "base_safe_mem");
    #[cfg(all(feature = "base_safe", feature = "safe_num"))]
    new_alias(&mut aliases, "base_safe_num");
    #[cfg(all(feature = "base_safe", feature = "safe_phys"))]
    new_alias(&mut aliases, "base_safe_phys");
    #[cfg(all(feature = "base_safe", feature = "safe_sys"))]
    new_alias(&mut aliases, "base_safe_sys");
    #[cfg(all(feature = "base_safe", feature = "safe_text"))]
    new_alias(&mut aliases, "base_safe_text");
    #[cfg(all(feature = "base_safe", feature = "safe_ui"))]
    new_alias(&mut aliases, "base_safe_ui");
    #[cfg(all(feature = "base_safe", feature = "safe_work"))]
    new_alias(&mut aliases, "base_safe_work");

    #[cfg(feature = "__dbg")]
    Build::println(&format!("Active compiler cfg flag aliases ({}): {:?}", aliases.len(), aliases));

    Ok(())
}
