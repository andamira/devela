// devela/build/main/alias.rs
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
    target_aliases(&mut aliases);

    #[cfg(feature = "__dbg")]
    Build::println(format!("Active compiler cfg flag aliases ({}): {:?}", aliases.len(), aliases));

    Ok(())
}

// 3
// any_target_arch_linux
// any_target_arch_riscv
// linux_syscall_target
fn target_aliases(aliases: &mut Vec<&'static str>) {
    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = var("CARGO_CFG_TARGET_OS").unwrap();

    let linux_arch =
        matches!(arch.as_str(), "x86" | "x86_64" | "arm" | "aarch64" | "riscv32" | "riscv64");

    if linux_arch {
        new_alias(aliases, "any_target_arch_linux");
    }

    if matches!(arch.as_str(), "riscv32" | "riscv64") {
        new_alias(aliases, "any_target_arch_riscv");
    }

    if linux_arch && matches!(os.as_str(), "linux" | "none") {
        new_alias(aliases, "linux_syscall_target");
    }
}
