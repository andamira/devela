// devela::build::alias
//
//! Aliases of combinations of configuration features.
//

#[cfg(feature = "__dbg")]
use super::Build;
use ::std::{env::var, io::Error as IoError};

/// Defines a cfg flag `alias` and saves its name in a `list`.
fn new_alias(list: &mut Vec<&'static str>, alias: &'static str) {
    println!("cargo:rustc-cfg={}", alias);
    list.push(alias);
}

pub(crate) fn main() -> Result<(), IoError> {
    #[cfg(feature = "__dbg")]
    Build::println_heading("Aliases:");

    let mut aliases = Vec::with_capacity(20);
    arch_aliases(&mut aliases);
    base_aliases(&mut aliases);

    #[cfg(feature = "__dbg")]
    Build::println(&format!("Active compiler cfg flag aliases ({}): {:?}", aliases.len(), aliases));

    Ok(())
}

// 2
// - any_target_arch_linux
// - any_target_arch_riscv
#[allow(unused_variables)]
fn arch_aliases(aliases: &mut Vec<&'static str>) {
    let target_arch = var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "x86"
        || target_arch == "x86_64"
        || target_arch == "arm"
        || target_arch == "aarch64"
        || target_arch == "riscv32"
        || target_arch == "riscv64"
    {
        new_alias(aliases, "any_target_arch_linux");
    }
    if target_arch == "riscv32" || target_arch == "riscv64" {
        new_alias(aliases, "any_target_arch_riscv");
    }
}

// 19
#[rustfmt::skip]
#[allow(unexpected_cfgs, unused_variables, clippy::ptr_arg)]
fn base_aliases(aliases: &mut Vec<&'static str>) {
    #[cfg(all(feature = "base_safe", feature = "safe"))]
    new_alias(aliases, "base_safe");

    #[cfg(all(feature = "base_safe", feature = "safe_build"))]
    new_alias(aliases, "base_safe_build");

    #[cfg(all(feature = "base_safe", feature = "safe_code"))]
    new_alias(aliases, "base_safe_code");

    #[cfg(all(feature = "base_safe", feature = "safe_data"))]
    new_alias(aliases, "base_safe_data");

    #[cfg(all(feature = "base_safe", feature = "safe_geom"))]
    new_alias(aliases, "base_safe_geom");

    #[cfg(all(feature = "base_safe", feature = "safe_lang"))]
    new_alias(aliases, "base_safe_lang");

    #[cfg(all(feature = "base_safe", feature = "safe_media"))]
    new_alias(aliases, "base_safe_media");
        #[cfg(all(feature = "base_safe", feature = "safe_audio"))]
        new_alias(aliases, "base_safe_audio");
        #[cfg(all(feature = "base_safe", feature = "safe_color"))]
        new_alias(aliases, "base_safe_color");
        #[cfg(all(feature = "base_safe", feature = "safe_draw"))]
        new_alias(aliases, "base_safe_draw");
        #[cfg(all(feature = "base_safe", feature = "safe_font"))]
        new_alias(aliases, "base_safe_font");
        #[cfg(all(feature = "base_safe", feature = "safe_image"))]
        new_alias(aliases, "base_safe_image");

    #[cfg(all(feature = "base_safe", feature = "safe_num"))]
    new_alias(aliases, "base_safe_num");

    #[cfg(all(feature = "base_safe", feature = "safe_phys"))]
    new_alias(aliases, "base_safe_phys");

    #[cfg(all(feature = "base_safe", feature = "safe_run"))]
    new_alias(aliases, "base_safe_run");

    #[cfg(all(feature = "base_safe", feature = "safe_sys"))]
    new_alias(aliases, "base_safe_sys");
        #[cfg(all(feature = "base_safe", feature = "safe_mem"))]
        new_alias(aliases, "base_safe_mem");

    #[cfg(all(feature = "base_safe", feature = "safe_text"))]
    new_alias(aliases, "base_safe_text");

    #[cfg(all(feature = "base_safe", feature = "safe_ui"))]
    new_alias(aliases, "base_safe_ui");

    #[cfg(all(feature = "base_safe", feature = "safe_work"))]
    new_alias(aliases, "base_safe_work");
}
