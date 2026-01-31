// devela::build::environment
//
//! Build script environment variables.
//
// NOTE: this file is shared between the build scripts in:
// - devela/build/main/
// - devela_base_core/build/
//
// https://doc.rust-lang.org/cargo/reference/environment-variables.html
// #environment-variables-cargo-sets-for-build-scripts

use std::{io::Error as IoError, path::Path};

pub(crate) fn main() -> Result<(), IoError> {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
    println!("cargo:rerun-if-env-changed=CARGO_PRIMARY_PACKAGE");
    println!("cargo:rerun-if-env-changed=CARGO_TARGET_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_WORKSPACE_DIR");

    // This environment variable will be set if the package being built is primary.
    let cargo_primary_package = option_env!("CARGO_PRIMARY_PACKAGE");
    if cargo_primary_package.is_some() {
        println!("cargo:rustc-cfg=cargo_primary_package");
    }

    // Makes sure `CARGO_TARGET_DIR` and `CARGO_WORKSPACE_DIR` are always defined.
    //
    // Used in: doclink! (/src/base/core/src/code/util/doclink
    // In sync with: /.cargo/config.toml#[env] & /src/base/core/build/environment.rs
    let cwd = get_workspace_dir();
    let ctd = get_target_dir(&cwd);
    println!("cargo:rustc-env=CARGO_WORKSPACE_DIR={}", cwd);
    println!("cargo:rustc-env=CARGO_TARGET_DIR={}", ctd);

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let is_workspace_member = matches!(
        crate_name.as_str(), // in sync with /Cargo.toml:members
        "devela_base_macros"
            | "devela_base_core"
            | "devela_base_alloc"
            | "devela_base_std"
            | "devela_macros"
            | "devela"
            | "devela_postbuild"
    );
    if is_workspace_member {
        println!("cargo:rustc-env=__DEVELA_MEMBER=");
        println!("cargo:rustc-env=__DEVELA_MEMBER_NAME={crate_name}");
    }

    #[cfg(feature = "__dbg")]
    {
        use super::{Build, CRATE_NAME};

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        Build::println_heading("Environment:");

        // using the global constant because if not, from here we can only get "build_script_mod"
        Build::println(&format!["· CARGO_CRATE_NAME={CRATE_NAME}"]);

        if let Some(v) = cargo_primary_package {
            Build::println(&format!["· CARGO_PRIMARY_PACKAGE={v}"]);
        } else {
            Build::println("x CARGO_PRIMARY_PACKAGE");
        }

        // Only show these env vars for the root package
        // if cargo_primary_package.is_some() { // FIXME: not set for deps
        if CRATE_NAME == "devela" {
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
                })
                .unwrap_or_else(|| ".".to_string())
        })
        .trim_end_matches('/')
        .to_string()
        + "/"
}
fn get_target_dir(workspace_dir: &str) -> String {
    option_env!("CARGO_TARGET_DIR")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}/target", workspace_dir))
        .trim_end_matches('/')
        .to_string()
        + "/"
}
