// devela::lib
//
//! A development foundation.
//

//* global config *//
//
// warnings:
#![warn(missing_docs, clippy::all)]
#![cfg_attr(
    not(all(doc, feature = "_docsrs_stable")),
    allow(rustdoc::broken_intra_doc_links)
)]
#![allow(
    unknown_lints,
    stable_features, // e.g. for associated_type_bounds
    clippy::empty_docs,
    clippy::doc_lazy_continuation, // don't be markdown picky
    clippy::mixed_attributes_style,
    //
    clippy::module_inception, // allow modules with the same name as its parent
    clippy::wrong_self_convention, // allow `is_` methods having an owned self
)]
#![deny(
    // WAIT: [lazy_type_alias](https://github.com/rust-lang/rust/issues/112792)
    type_alias_bounds,
)]
//
// environment:
#![cfg_attr(not(feature = "std"), no_std)]
//
// safety:
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly:
// WAIT: [doc_cfg](https://github.com/rust-lang/rust/issues/43781)
// WAIT: [doc_notable_trait](https://github.com/rust-lang/rust/issues/45040)
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg, doc_notable_trait))]
// WAIT: [coroutines](https://github.com/rust-lang/rust/issues/43122)
#![cfg_attr(
    feature = "nightly_coro",
    feature(coroutines, coroutine_trait, iter_from_coroutine)
)]
// WAIT: [portable_simd](https://github.com/rust-lang/rust/issues/86656)
#![cfg_attr(feature = "nightly_simd", feature(portable_simd))]
// #![cfg_attr(
//     feature = "nightly_stabilized",
//     feature()
// )]

// safeguard environment:
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
//
// safeguard safety:
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", // includes all below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_const",
        feature = "unsafe_dyn", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// but you can enable `safe_*` features to prevent `unsafe` use in specific modules.

pub mod code;
pub mod data;
pub mod exec;
pub mod mem;
pub mod num;
pub mod sys;
pub mod text;
pub mod time;

/// All the library items.
pub mod all {
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::all::*,
        data::all::*,
        exec::all::*,
        mem::all::*,
        num::all::*,
        sys::all::*,
        text::all::*,
        time::all::*,
    };
}
#[doc(no_inline)]
#[allow(unused_imports)]
pub use all::*;

/// Dependencies.
pub mod _deps;

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library environment.*
#[doc(inline)]
#[cfg(feature = "alloc")]
pub extern crate alloc as _liballoc;

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library environment.*
#[doc(inline)]
pub use ::core as _libcore;

/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library environment.*
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std as _libstd;

/// Information about the library
pub mod _info {
    /// Documented examples.
    #[cfg(any(doc, test))]
    pub mod examples;

    /// Cargo features.
    pub mod features {
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./_info/features.md")]
    }
}
