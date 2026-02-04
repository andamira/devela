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
