// devela build::utils
//
//! build script environment variables
//

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    {
        use crate::utils::{println_heading, println_var};

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
        println_heading("Environment variables:");

        println_var("CARGO_MANIFEST_DIR");
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
        //     crate::utils::println(&format![">> {key}: {value}"]);
        // }
    }

    Ok(())
}
