//
//! Aliases of combinations of configuration features.
//

#[cfg(feature = "__dbg")]
use super::Build;
use ::std::env::{var, var_os};
use ::std::io::Error as IoError;

/// Defines a cfg flag `alias` and saves its name in a `list`.
fn new_alias(list: &mut Vec<&'static str>, alias: &'static str) {
    println!("cargo:rustc-cfg={}", alias);
    list.push(alias);
}

pub(crate) fn main() -> Result<(), IoError> {
    #[cfg(feature = "__dbg")]
    Build::println_heading("Aliases:");

    let mut aliases = Vec::with_capacity(20);
    module_aliases(&mut aliases); // 12
    target_aliases(&mut aliases); // 13

    #[cfg(feature = "__dbg")]
    Build::println(format!("Active compiler cfg flag aliases ({}): {:?}", aliases.len(), aliases));

    Ok(())
}

// 14
// root_[code|data|error|geom|lang|media|num|phys|run|sys|text|ui|work|yard]
fn module_aliases(aliases: &mut Vec<&'static str>) {
    let selective = var_os("CARGO_FEATURE___SELECT_MODULES").is_some();
    let mut any_root = false;

    for (feature, alias) in [
        ("CARGO_FEATURE_CODE", "root_code"),
        ("CARGO_FEATURE_DATA", "root_data"),
        ("CARGO_FEATURE_ERROR", "root_error"),
        ("CARGO_FEATURE_GEOM", "root_geom"),
        ("CARGO_FEATURE_LANG", "root_lang"),
        ("CARGO_FEATURE_MEDIA", "root_media"),
        ("CARGO_FEATURE_NUM", "root_num"),
        ("CARGO_FEATURE_PHYS", "root_phys"),
        ("CARGO_FEATURE_RUN", "root_run"),
        ("CARGO_FEATURE_SYS", "root_sys"),
        ("CARGO_FEATURE_TEXT", "root_text"),
        ("CARGO_FEATURE_UI", "root_ui"),
        ("CARGO_FEATURE_WORK", "root_work"),
    ] {
        if !selective || var_os(feature).is_some() {
            new_alias(aliases, alias);
            any_root = true;
        }
    }

    // Internal scaffolding, not a user-facing feature.
    println!("cargo:rerun-if-env-changed=DEVELA_BENCH_YARD");
    if any_root || var_os("DEVELA_BENCH_YARD").is_some() {
        new_alias(aliases, "root_yard");
    }
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
