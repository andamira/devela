// devela::build::environment
//
//! Build script environment variables.
//
// NOTE: this file is shared between the build scripts in:
// - devela/build/main/
// - devela_base/build/
//
// https://doc.rust-lang.org/cargo/reference/environment-variables.html
// #environment-variables-cargo-sets-for-build-scripts

#[cfg(feature = "__dbg")]
use ::std::sync::LazyLock;

#[cfg(feature = "__dbg")]
/// The crate name to compiled after running the current build script.
pub(crate) static CRATE_NAME: LazyLock<&str> = LazyLock::new(|| include_str!("crate_name").trim());

pub(crate) fn main() -> Result<(), std::io::Error> {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
    println!("cargo:rerun-if-env-changed=CARGO_PRIMARY_PACKAGE");
    println!("cargo:rerun-if-env-changed=CARGO_TARGET_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_WORKSPACE_DIR");

    // This environment variable will be set if the package being built is primary.
    let cargo_primary_package = option_env!("CARGO_PRIMARY_PACKAGE");
    if cargo_primary_package.is_some() {
        println!("cargo:rustc-cfg=cargo_primary_package");
    }

    // Makes sure CARGO_TARGET_DIR is always defined, with trailing slash.
    //
    // Used in: _doclink!
    // In sync with: /.cargo/config.toml#[env] & /libs/base/build/environment.rs
    let ctd =
        option_env!("CARGO_TARGET_DIR").unwrap_or(concat![env!("CARGO_WORKSPACE_DIR"), "target/"]);
    let cargo_target_dir = if ctd.ends_with('/') { ctd.to_string() } else { format!("{}/", ctd) };
    println!("cargo:rustc-env=CARGO_TARGET_DIR={}", cargo_target_dir);

    #[cfg(feature = "__dbg")]
    {
        use super::Build;

        Build::println(&format!["CRATE_NAME = {CRATE_NAME:?}"]);

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        Build::println_heading("Environment variables:");

        if let Some(v) = cargo_primary_package {
            Build::println(&format!["· CARGO_PRIMARY_PACKAGE={v}"]);
        } else {
            Build::println("x CARGO_PRIMARY_PACKAGE");
        }

        // Only show these env vars for the root package
        // if cargo_primary_package.is_some() { // FIXME: not set for deps
        if *CRATE_NAME == "devela" {
            Build::println_var("CARGO_WORKSPACE_DIR");
            Build::println_var("CARGO_TARGET_DIR");
            Build::println_var("CARGO_MANIFEST_DIR");
            Build::println_var("CARGO_MANIFEST_PATH");
            Build::println_var("OUT_DIR");

            Build::println_var("HOST");
            Build::println_var("TARGET");
            Build::println_var("PROFILE");
            Build::println_var("NUM_JOBS");

            Build::println_var("RUSTC");
            // Build::println_var("RUSTC_WRAPPER");
            // Build::println_var("RUSTC_LINKER");
            // Build::println_var("RUSTDOC");
            // Build::println_var("CARGO");
            // Build::println_var("CARGO_MAKEFLAGS");
            // Build::println_var("CARGO_ENCODED_RUSTFLAGS");

            // DEBUG
            // for (key, value) in std::env::vars() {
            //     Build::println(&format![">> {key}: {value}"]);
            // }
        }

        Build::println_var_encoded("CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS");
        Build::println_var("RUSTDOCFLAGS");
    }

    Ok(())
}
