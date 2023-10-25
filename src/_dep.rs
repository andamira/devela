// devela::dep
//
//!
//! # Dependencies
//!
//! Several minimal optional dependencies are included, providing useful
//! functionality missing from the standard library.
//!
//! # Features
//!
//! Enabling the `dep` feature will automatically enable
//! the optional dependencies associated to any enabled root modules.
//!
//! For example, enabling `dep` and [`task`], will automatically enable
//! the `atomic` and `portable-atomic` dependencies; while by leaving `dep`
//! disabled, you could enable just the `task` and `atomic` dependencies.
//!
//! In any case, manually enabled optional dependency will remain enabled.
//!
//! The `dep` feature takes precedence, so any dependency that imports the
//! `devela` crate with the `dep` feature enabled, will also enable the
//! optional dependencies associated to any enabled modules.
//!
//! For example, enabling `dep` and [`result`][crate::result] will not enable
//! the [`option_unwrap`][crate::result::option_unwrap] function unless the
//! [`text`] module is also enabled; or if the `const-str` dependency
//! is enabled independently of the `text` module.
//

pub use _dep::*;

// When the `dep` feature is disabled, any optional dependency can be
// enabled manually and independently of any other enabled module.
#[cfg(not(feature = "dep"))]
mod _dep {
    #[cfg(feature = "atomic")]
    pub use ::atomic;

    #[cfg(feature = "az")]
    pub use ::az;

    #[cfg(feature = "bytemuck")]
    pub use ::bytemuck;

    #[cfg(feature = "const-str")]
    pub use ::const_str;

    #[cfg(feature = "devela_macros")]
    pub use ::devela_macros;

    #[cfg(all(feature = "hashbrown", feature = "alloc"))]
    pub use ::hashbrown;

    #[cfg(feature = "libm")]
    pub use ::libm;

    #[cfg(feature = "portable-atomic")]
    pub use ::portable_atomic;

    #[cfg(feature = "unicode-segmentation")]
    pub use ::unicode_segmentation;
}

// When the `dep` feature is enabled, the `devela_depend` crate is enabled,
// and the enabled modules will enable their associated optional dependencies.
//
// Any independently enabled optional dependency will also be enabled.
//
// This is also used for documentation.
#[cfg(feature = "dep")]
mod _dep {
    use crate::meta::reexport;

    reexport! { depend feature: "task",
    dep: "atomic", atomic, "A generic atomic wrapper type." }

    reexport! { depend feature: "convert",
    dep: "az", az, "Casts and checked casts." }

    reexport! { depend feature: "mem",
    dep: "bytemuck", bytemuck, "Small utilities for casting between plain data types." }

    reexport! { depend feature: "text",
    dep: "const-str", const_str, "Compile-time string operations." }

    reexport! { depend feature: "meta",
    dep: "devela_macros", devela_macros, "Procedural macros for `devela`." }

    reexport! { depend feature: "no_std",
    dep: "libm", libm, "A port of [`MUSL`](https://musl.libc.org/)'s libm to Rust." }

    reexport! { depend feature: "data", also: "alloc",
    dep: "hashbrown", hashbrown,
    "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`." }

    reexport! { depend feature: "task",
    dep: "portable-atomic", portable_atomic,
    "Portable atomic types including 128-bit atomics, floats, etc." }

    reexport! { depend feature: "text",
    dep: "unicode-segmentation", unicode_segmentation,
    "Split strings on Grapheme Cluster, Word or Sentence boundaries." }
}
