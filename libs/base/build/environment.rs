// devela_base::build::environment
//
//! Build script environment variables.
//

pub(crate) fn main() -> Result<(), std::io::Error> {
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
    // In sync with: /.cargo/config.toml#[env] & /build/main/environment.rs
    let ctd =
        option_env!("CARGO_TARGET_DIR").unwrap_or(concat![env!("CARGO_WORKSPACE_DIR"), "target/"]);
    let cargo_target_dir = if ctd.ends_with('/') { ctd.to_string() } else { format!("{}/", ctd) };
    println!("cargo:rustc-env=CARGO_TARGET_DIR={}", cargo_target_dir);

    Ok(())
}
