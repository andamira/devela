// devela build::utils
//
//! build script environment variables
//

pub(crate) fn main() -> Result<(), std::io::Error> {
    let cargo_primary_package = option_env!("CARGO_PRIMARY_PACKAGE");
    if cargo_primary_package.is_some() {
        println!("cargo:rustc-cfg=cargo_primary_package");
    }

    #[cfg(feature = "__dbg")]
    {
        use crate::utils::{println, println_heading, println_var};

        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        println_heading("Environment variables:");

        if let Some(v) = cargo_primary_package {
            println(&format!["· CARGO_PRIMARY_PACKAGE={v}"]);
        } else {
            println(&format!["x CARGO_PRIMARY_PACKAGE"]);
        }

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
