// devela_base::build::environment
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

    Ok(())
}
