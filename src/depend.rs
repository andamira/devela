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
//! [`str`][mod@str] module is also enabled; or if the `const-str` dependency
//! is enabled independently of the `str` module.
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

    #[cfg(feature = "portable-atomic")]
    pub use ::portable_atomic;

    #[cfg(feature = "unicode-segmentation")]
    pub use ::unicode_segmentation;
}

// When the `depend` feature is enabled, the `devela_depend` crate is enabled,
// and the enabled modules will enable their associated optional dependencies.
//
// Any independently enabled optional dependency will also be enabled.
#[cfg(feature = "depend")]
mod depend {
    #[cfg(all(feature = "atomic", not(feature = "sync")))]
    pub use ::atomic;
    /// <span class="stab portability" title="re-exported `atomic` crate">`atomic`</span>
    #[doc = "A generic atomic wrapper type.\n\n"]
    #[doc = "*Re-exported [`atomic`](https://docs.rs/atomic)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
    pub use depend::atomic;

    #[cfg(all(feature = "az", not(feature = "convert")))]
    pub use ::az;
    /// <span class="stab portability" title="re-exported `az` crate">`az`</span>
    #[doc = "Casts and checked casts.\n\n"]
    #[doc = "*Re-exported [`az`](https://docs.rs/az)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "convert")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "convert")))]
    pub use depend::az;

    #[cfg(all(feature = "bytemuck", not(feature = "mem")))]
    pub use ::bytemuck;
    /// <span class="stab portability" title="re-exported `bytemuck` crate">`bytemuck`</span>
    #[doc = "Small utilities for casting between plain data types.\n\n"]
    #[doc = "*Re-exported [`bytemuck`](https://docs.rs/bytemuck)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "mem")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
    pub use depend::bytemuck;

    #[cfg(all(feature = "const-str", not(feature = "str")))]
    pub use ::const_str;
    /// <span class="stab portability" title="re-exported `const-str` crate">`const-str`</span>
    #[doc = "Compile-time string operations.\n\n"]
    #[doc = "*Re-exported [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "str")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "str")))]
    pub use depend::const_str;

    #[cfg(all(feature = "devela_macros", not(feature = "codegen")))]
    pub use ::devela_macros;
    /// <span class="stab portability" title="re-exported `devela_macros`
    /// crate">`devela_macros`</span>
    #[doc = "Procedural macros for `devela`.\n\n"]
    #[doc = "*Re-exported [`devela_macros`](https://docs.rs/devela_macros)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "codegen")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "codegen")))]
    pub use depend::devela_macros;

    #[cfg(all(feature = "portable-atomic", not(feature = "sync")))]
    pub use ::portable_atomic;
    /// <span class="stab portability" title="re-exported `portable-atomic`
    /// crate">`portable-atomic`</span>
    #[doc = "Portable atomic types including 128-bit atomics, floats, etc.\n\n"]
    #[doc = "*Re-exported [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
    pub use depend::portable_atomic;

    #[cfg(all(feature = "unicode_segmentation", not(feature = "string")))]
    pub use ::unicode_segmentation;
    /// <span class="stab portability" title="re-exported `unicode-segmentation`
    /// crate">`unicode-segmentation`</span>
    #[doc = "Split strings on Grapheme Cluster, Word or Sentence boundaries.\n\n"]
    #[doc = "*Re-exported [`unicode-segmentation`](https://docs.rs/unicode-segmentation)*
    crate.\n\n---"]
    #[doc(inline)]
    #[cfg(feature = "string")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "string")))]
    pub use depend::unicode_segmentation;
}
