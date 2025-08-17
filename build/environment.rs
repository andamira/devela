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
        use super::utils::{println, println_heading, println_var, println_var_encoded};

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        println_heading("Environment variables:");

        if let Some(v) = cargo_primary_package {
            println(&format!["· CARGO_PRIMARY_PACKAGE={v}"]);
        } else {
            println("x CARGO_PRIMARY_PACKAGE");
        }

        println_var("CARGO_MANIFEST_DIR");
        println_var("CARGO_MANIFEST_PATH");
        println_var("OUT_DIR");

        println_var("HOST");
        println_var("TARGET");
        println_var("PROFILE");
        println_var("NUM_JOBS");

        println_var("RUSTC");
        // println_var("RUSTC_WRAPPER");
        // println_var("RUSTC_LINKER");
        // println_var("RUSTDOC");
        // println_var("CARGO");
        // println_var("CARGO_MAKEFLAGS");
        // println_var("CARGO_ENCODED_RUSTFLAGS");

        // for (key, value) in std::env::vars() {
        //     super::utils::println(&format![">> {key}: {value}"]);
        // }

        println_var_encoded("CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS");
        println_var("RUSTDOCFLAGS");
    }

    Ok(())
}
