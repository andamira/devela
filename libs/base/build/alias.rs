// devela_base::build::alias
//
//! Aliases of combinations of configuration features.
//

pub(crate) fn main() -> Result<(), std::io::Error> {

    #[cfg(all(feature = "base_safe", feature = "safe"))]
    println!("cargo:rustc-cfg=base_safe");

    #[cfg(all(feature = "base_safe", feature = "safe_build"))]
    println!("cargo:rustc-cfg=base_safe_build");

    #[cfg(all(feature = "base_safe", feature = "safe_code"))]
    println!("cargo:rustc-cfg=base_safe_code");
    #[cfg(all(feature = "base_safe", feature = "safe_data"))]
    println!("cargo:rustc-cfg=base_safe_data");
    #[cfg(all(feature = "base_safe", feature = "safe_game"))]
    println!("cargo:rustc-cfg=base_safe_game");
    #[cfg(all(feature = "base_safe", feature = "safe_lang"))]
    println!("cargo:rustc-cfg=base_safe_lang");
    #[cfg(all(feature = "base_safe", feature = "safe_media"))]
    println!("cargo:rustc-cfg=base_safe_media");
    #[cfg(all(feature = "base_safe", feature = "safe_mem"))]
    println!("cargo:rustc-cfg=base_safe_mem");
    #[cfg(all(feature = "base_safe", feature = "safe_num"))]
    println!("cargo:rustc-cfg=base_safe_num");
    #[cfg(all(feature = "base_safe", feature = "safe_phys"))]
    println!("cargo:rustc-cfg=base_safe_phys");
    #[cfg(all(feature = "base_safe", feature = "safe_sys"))]
    println!("cargo:rustc-cfg=base_safe_sys");
    #[cfg(all(feature = "base_safe", feature = "safe_text"))]
    println!("cargo:rustc-cfg=base_safe_text");
    #[cfg(all(feature = "base_safe", feature = "safe_ui"))]
    println!("cargo:rustc-cfg=base_safe_ui");
    #[cfg(all(feature = "base_safe", feature = "safe_work"))]
    println!("cargo:rustc-cfg=base_safe_work");

    Ok(())
}
