// devela::depend
//
//!
//! # Dependencies
//!
//! Several minimal optional dependencies are included, providing useful
//! functionality missing from the standard library.
//!
//! # Features
//!
//! Enabling the `depend` feature will automatically enable
//! the optional dependencies associated to any enabled root modules.
//!
//! For example, enabling `depend` and [`sync`], will automatically enable
//! the `atomic` and `portable-atomic` dependencies; while by leaving `depend`
//! disabled, you could enable just the `sync` and `atomic` dependencies.
//!
//! In any case, manually enabled optional dependency will remain enabled.
//!
//! The `depend` feature takes precedence, so any dependency that imports the
//! `devela` crate with the `depend` feature enabled, will also enable the
//! optional dependencies associated to any enabled modules.
//!
//! For example, enabling `depend` and [`option`][crate::option] will not enable
//! the [`option_unwrap`][crate::option::option_unwrap] function unless the
//! [`string`] module is also enabled; or if the `const-str` dependency
//! is enabled independently of the `string` module.
//

pub use depend::*;

// When the `depend` feature is disabled, any optional dependency can be
// enabled manually and independently of any other enabled module.
#[cfg(not(feature = "depend"))]
mod depend {
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

    #[cfg(feature = "portable-atomic")]
    pub use ::portable_atomic;

    #[cfg(feature = "unicode-segmentation")]
    pub use ::unicode_segmentation;
}

// When the `depend` feature is enabled, the `devela_depend` crate is enabled,
// and the enabled modules will enable their associated optional dependencies.
//
// Any independently enabled optional dependency will also be enabled.
//
// This is also used for documentation.
#[cfg(feature = "depend")]
mod depend {
    use crate::codegen::reexport;

    reexport! { depend feature: "sync",
    dep: "atomic", atomic, "A generic atomic wrapper type." }

    reexport! { depend feature: "convert",
    dep: "az", az, "Casts and checked casts." }

    reexport! { depend feature: "mem",
    dep: "bytemuck", bytemuck, "Small utilities for casting between plain data types." }

    reexport! { depend feature: "string",
    dep: "const-str", const_str, "Compile-time string operations." }

    reexport! { depend feature: "codegen",
    dep: "devela_macros", devela_macros, "Procedural macros for `devela`." }

    reexport! { depend feature: "collections", also: "alloc",
    dep: "hashbrown", hashbrown,
    "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`." }

    reexport! { depend feature: "sync",
    dep: "portable-atomic", portable_atomic,
    "Portable atomic types including 128-bit atomics, floats, etc." }

    reexport! { depend feature: "string",
    dep: "unicode-segmentation", unicode_segmentation,
    "Split strings on Grapheme Cluster, Word or Sentence boundaries." }
}
