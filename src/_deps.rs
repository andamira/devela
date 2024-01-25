// devela::_deps
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
//! For example, enabling `dep` and [`work`], will automatically enable
//! the `atomic` and `portable-atomic` dependencies; while by leaving `dep`
//! disabled, you could enable just the `work` and `atomic` dependencies.
//!
//! In any case, manually enabled optional dependency will remain enabled.
//!
//! The `dep` feature takes precedence, so any dependency that imports the
//! `devela` crate with the `dep` feature enabled, will also enable the
//! optional dependencies associated to any enabled modules.
//!
//! For example, enabling `dep` and [`error`][crate::error] will not enable
//! the [`option_unwrap`][crate::error::option_unwrap] function unless the
//! [`text`] module is also enabled; or if the `const-str` dependency
//! is enabled independently of the `text` module.
//

#![cfg_attr(not(feature = "full"), allow(rustdoc::private_intra_doc_links))]

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library environment.*
#[doc(inline)]
#[cfg(feature = "alloc")]
pub extern crate alloc;
/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library environment.*
#[doc(inline)]
pub use ::core;
/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library environment.*
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std;

#[allow(unused)]
pub use _dep::*;

// When the `dep` feature is disabled, any optional dependency can be
// enabled manually and independently of any other enabled module.
#[cfg(not(feature = "dep"))]
mod _dep {
    #[cfg(feature = "atomic")]
    pub use ::atomic;

    #[cfg(feature = "bytemuck")]
    pub use ::bytemuck;

    #[cfg(feature = "const-str")]
    pub use ::const_str;

    #[cfg(all(feature = "hashbrown", feature = "alloc"))]
    pub use ::hashbrown;

    #[cfg(feature = "libm")]
    pub use ::libm;

    #[cfg(feature = "portable-atomic")]
    pub use ::portable_atomic;

    #[cfg(feature = "unicode-segmentation")]
    pub use ::unicode_segmentation;

    #[cfg(feature = "unicode-width")]
    pub use ::unicode_width;
}

// When the `dep` feature is enabled, the `devela_depend` crate is enabled,
// and the enabled modules will enable their associated optional dependencies.
//
// Any independently enabled optional dependency will also be enabled.
//
// This is also used for documentation.
#[cfg(feature = "dep")]
mod _dep {
    use crate::code::reexport;

    reexport! { depend any_features: "work",
    dep: "atomic", atomic, "A generic atomic wrapper type." }

    reexport! { depend any_features: "data", "mem",
    dep: "bytemuck", bytemuck, "Small utilities for casting between plain data types." }

    reexport! { depend any_features: "os_term", "text",
    dep: "const-str", const_str, "Compile-time string operations." }

    reexport! { depend any_features: "no_std",
    dep: "libm", libm, "A port of [`MUSL`](https://musl.libc.org/)'s libm to Rust." }

    reexport! { depend any_features: "data", all_features: "alloc",
    dep: "hashbrown", hashbrown,
    "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`." }

    reexport! { depend any_features: "work",
    dep: "portable-atomic", portable_atomic,
    "Portable atomic types including 128-bit atomics, floats, etc." }

    reexport! { depend any_features: "text",
    dep: "unicode-segmentation", unicode_segmentation,
    "Split strings on Grapheme Clusters, Words or Sentences." }

    reexport! { depend any_features: "text",
    dep: "unicode-width", unicode_width,
    "Determine displayed width of `char` and `str` types." }
}
