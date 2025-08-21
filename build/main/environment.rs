// devela::build::environment
//
//! Build script environment variables.
//
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

pub(crate) fn main() -> Result<(), std::io::Error> {
    // This environment variable will be set if the package being built is primary.
    let cargo_primary_package = option_env!("CARGO_PRIMARY_PACKAGE");
    if cargo_primary_package.is_some() {
        println!("cargo:rustc-cfg=cargo_primary_package");
    }

    #[cfg(feature = "__dbg")]
    {
        use super::Build;

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        Build::println_heading("Environment variables:");

        if let Some(v) = cargo_primary_package {
            Build::println(&format!["· CARGO_PRIMARY_PACKAGE={v}"]);
        } else {
            Build::println("x CARGO_PRIMARY_PACKAGE");
        }

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

        // for (key, value) in std::env::vars() {
        //     Build::println(&format![">> {key}: {value}"]);
        // }

        Build::println_var_encoded("CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS");
        Build::println_var("RUSTDOCFLAGS");
    }

    Ok(())
}
