// devela::build::main::environment
//
//! Build-script environment variables.
//!
//! Emits stable compile-time environment values used by the crate, including:
//! - workspace and target directories,
//! - devela workspace-member markers,
//! - target cfg mirrors for `no_std`-available environment constants.
//!
//! Cargo exposes target cfg values to build scripts through `CARGO_CFG_*`.
//
// https://doc.rust-lang.org/cargo/reference/environment-variables.html
// #environment-variables-cargo-sets-for-build-scripts

use super::Build;
use std::{env, io::Error as IoError, path::Path};

/* entry point */

pub(crate) fn main() -> Result<(), IoError> {
    emit_rerun_markers();
    emit_primary_package_flag();
    emit_workspace_env();
    emit_member_env();
    emit_target_env();

    #[cfg(feature = "__dbg")]
    print_debug_environment();

    Ok(())
}

/* rerun markers */

/// Marks environment variables that should cause this build script to rerun.
fn emit_rerun_markers() {
    Build::rerun_if_env_changed("CARGO_PRIMARY_PACKAGE");
    Build::rerun_if_env_changed("CARGO_TARGET_DIR");
    Build::rerun_if_env_changed("CARGO_WORKSPACE_DIR");
}

/// Emits the `cargo_primary_package` cfg flag when this package is primary.
fn emit_primary_package_flag() {
    let is_primary = option_env!("CARGO_PRIMARY_PACKAGE");
    Build::emit_flag_if("cargo_primary_package", is_primary.is_some());
}

/* workspace environment */

/// Emits normalized workspace and target directory variables.
fn emit_workspace_env() {
    // Used in: doclink! (/src/code/util/doclink.rs)
    // In sync with: /.cargo/config.toml#[env]
    let workspace_dir = get_workspace_dir();
    let target_dir = get_target_dir(&workspace_dir);
    Build::emit_env("CARGO_WORKSPACE_DIR", workspace_dir);
    Build::emit_env("CARGO_TARGET_DIR", target_dir);
}

/// Returns the configured workspace directory,
/// or the nearest Cargo root, with a trailing slash.
fn get_workspace_dir() -> String {
    option_env!("CARGO_WORKSPACE_DIR")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            option_env!("CARGO_MANIFEST_DIR")
                .and_then(|manifest_dir| {
                    let path = Path::new(manifest_dir);
                    path.ancestors()
                        .find(|p| p.join("Cargo.toml").exists())
                        .and_then(|p| p.to_str())
                        .map(|s| s.to_string())
                    // Or take the outermost ancestor containing a Cargo.toml.
                    // path.ancestors()
                    //     .filter(|p| p.join("Cargo.toml").exists())
                    //     .last()
                    //     .and_then(|p| p.to_str())
                    //     .map(|s| s.to_string())
                })
                .unwrap_or_else(|| ".".to_string())
        })
        .trim_end_matches('/')
        .to_string()
        + "/"
}
/// Returns the cargo target directory with a trailing slash.
fn get_target_dir(workspace_dir: &str) -> String {
    option_env!("CARGO_TARGET_DIR")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{workspace_dir}target"))
        .trim_end_matches('/')
        .to_string()
        + "/"
}

/* devela member environment */

/// Emits devela workspace-member markers for crates known to this workspace.
fn emit_member_env() {
    let crate_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set");
    // In sync with /Cargo.toml:workspace.members
    let is_workspace_member =
        matches!(crate_name.as_str(), "devela" | "devela_macros" | "devela_postbuild");
    if is_workspace_member {
        Build::emit_env_marker("__DEVELA_MEMBER");
        Build::emit_env("__DEVELA_MEMBER_NAME", crate_name);
    }
}

/* target environment */

/// Target cfg values exposed by Cargo to the build script.
struct TargetCfg {
    target: String,
    arch: String,
    os: String,
    vendor: String,
    target_env: String,
    abi: String,
    families: String,
    endian: String,
    pointer_width: String,
}
impl TargetCfg {
    /// Reads the target cfg environment provided by Cargo.
    fn read() -> Self {
        Self {
            target: required_env("TARGET"),
            arch: required_env("CARGO_CFG_TARGET_ARCH"),
            os: required_env("CARGO_CFG_TARGET_OS"),
            vendor: required_env("CARGO_CFG_TARGET_VENDOR"),
            target_env: optional_env("CARGO_CFG_TARGET_ENV"),
            abi: optional_env("CARGO_CFG_TARGET_ABI"),
            families: optional_env("CARGO_CFG_TARGET_FAMILY"),
            endian: required_env("CARGO_CFG_TARGET_ENDIAN"),
            pointer_width: required_env("CARGO_CFG_TARGET_POINTER_WIDTH"),
        }
    }
}

/// Emits devela target constants as compile-time environment variables.
// In sync with: /src/sys/env/namespace.rs
fn emit_target_env() {
    let target = TargetCfg::read();
    Build::emit_env("DEVELA_TARGET", &target.target);
    Build::emit_env("DEVELA_TARGET_ARCH", &target.arch);
    Build::emit_env("DEVELA_TARGET_OS", &target.os);
    Build::emit_env("DEVELA_TARGET_VENDOR", &target.vendor);
    Build::emit_env("DEVELA_TARGET_ENV", &target.target_env);
    Build::emit_env("DEVELA_TARGET_ABI", &target.abi);
    Build::emit_env("DEVELA_TARGET_FAMILIES", &target.families);
    Build::emit_env("DEVELA_TARGET_FAMILY", primary_family(&target.families));
    Build::emit_env("DEVELA_TARGET_ENDIAN", &target.endian);
    Build::emit_env("DEVELA_TARGET_POINTER_WIDTH", &target.pointer_width);
    let (dll_prefix, dll_extension, exe_extension) = filename_parts(&target);
    Build::emit_env("DEVELA_DLL_PREFIX", dll_prefix);
    Build::emit_env("DEVELA_DLL_EXTENSION", dll_extension);
    Build::emit_env("DEVELA_DLL_SUFFIX", suffix_from_ext(dll_extension));
    Build::emit_env("DEVELA_EXE_EXTENSION", exe_extension);
    Build::emit_env("DEVELA_EXE_SUFFIX", suffix_from_ext(exe_extension));
}
/// Reads a required environment variable, panicking if Cargo did not provide it.
fn required_env(var: &str) -> String {
    env::var(var).unwrap_or_else(|_| panic!("{var} not set"))
}
/// Reads an optional environment variable, returning `""` when absent.
fn optional_env(var: &str) -> String {
    env::var(var).unwrap_or_default()
}
/// Returns whether `families` contains the given comma-separated target family.
fn has_family(families: &str, family: &str) -> bool {
    families.split(',').any(|f| f.trim() == family)
}

/// Returns the primary std-compatible target family.
///
/// `target_family` can contain multiple values. This chooses one stable,
/// std-like value for `Env::FAMILY`, while `Env::TARGET_FAMILIES` keeps all.
fn primary_family(families: &str) -> &'static str {
    if has_family(families, "windows") {
        "windows"
    } else if has_family(families, "unix") {
        "unix"
    } else if has_family(families, "wasm") {
        "wasm"
    } else if has_family(families, "itron") {
        "itron"
    } else {
        ""
    }
}
/// Returns std-like dynamic-library and executable filename extensions.
///
/// This is derived from Cargo target cfg values. It is intentionally conservative
/// for unknown/custom targets, where the empty string is safer than inventing a
/// platform convention.
fn filename_parts(target: &TargetCfg) -> (&'static str, &'static str, &'static str) {
    match target.os.as_str() {
        // Windows PE/COFF.
        "windows" => ("", "dll", "exe"),
        // Apple platforms.
        "macos" | "ios" | "tvos" | "watchos" | "visionos" => ("lib", "dylib", ""),
        // Fortanix SGX.
        "fortanix" => ("lib", "sgxs", "sgxs"),
        // WASM-like targets.
        "wasi" => ("", "wasm", "wasm"),
        "emscripten" => ("", "wasm", "js"),
        // UEFI applications conventionally use `.efi`.
        "uefi" => ("", "", "efi"),
        // Most Unix-like targets use `lib*.so` for dynamic libraries
        // and no executable suffix.
        _ if has_family(&target.families, "unix") => ("lib", "so", ""),
        // Bare-metal, custom, or unknown targets.
        _ => ("", "", ""),
    }
}
/// Returns a filename suffix for an extension, including the leading dot.
fn suffix_from_ext(ext: &str) -> String {
    if ext.is_empty() { String::new() } else { format!(".{ext}") }
}

/* debug */

/// Prints selected build-script environment values when `__dbg` is enabled.
#[cfg(feature = "__dbg")]
fn print_debug_environment() {
    use super::CRATE_NAME;

    Build::println_heading("Environment:");

    // Uses the global constant because from here Cargo may expose the build script crate name.
    Build::println(format!["· CARGO_CRATE_NAME={CRATE_NAME}"]);

    if let Ok(v) = env::var("CARGO_PRIMARY_PACKAGE") {
        Build::println(format!["· CARGO_PRIMARY_PACKAGE={v}"]);
    } else {
        Build::println("x CARGO_PRIMARY_PACKAGE");
    }
    if CRATE_NAME == "devela" {
        for var in [
            "CARGO_WORKSPACE_DIR",
            "CARGO_TARGET_DIR",
            "CARGO_MANIFEST_DIR",
            "CARGO_MANIFEST_PATH",
            "OUT_DIR",
            "HOST",
            "TARGET",
            "PROFILE",
            "NUM_JOBS",
            "RUSTC",
            "CARGO_CFG_TARGET_ARCH",
            "CARGO_CFG_TARGET_OS",
            "CARGO_CFG_TARGET_VENDOR",
            "CARGO_CFG_TARGET_ENV",
            "CARGO_CFG_TARGET_ABI",
            "CARGO_CFG_TARGET_FAMILY",
            "CARGO_CFG_TARGET_ENDIAN",
            "CARGO_CFG_TARGET_POINTER_WIDTH",
        ] {
            Build::println_var(var);
        }
    }
    Build::println_var_encoded("CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS");
    Build::println_var("RUSTDOCFLAGS");
}
