// devela::lib
//
//! A cohesive & modular development layer, designed for clarity and low-level control.
//

//* global config *//
//
// lints: (deny, warn, allow)
// - https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/builtin/index.html#constants
// - https://rust-lang.github.io/rust-clippy/rust-1.80.0/index.html
// - https://doc.rust-lang.org/rustdoc/lints.html
#![deny(
    // WAIT: [lazy_type_alias](https://github.com/rust-lang/rust/issues/112792)
    type_alias_bounds, // detects bounds in type aliases
    unsafe_op_in_unsafe_fn, // unsafe operations in unsafe functions without explicit unsafe block
    clippy::missing_safety_doc, // deny if there's no # Safety section in public unsafe fns
)]
#![warn(
    missing_docs, // missing docs for public items
    clippy::all, // (the default set of clippy lints)
    // from clippy::pedantic:
    clippy::bool_to_int_with_if, // using an if statement to convert a bool to an int
    clippy::cloned_instead_of_copied, // usage of cloned() where copied() could be used
    clippy::default_union_representation, // union declared without #[repr(C)]
    clippy::empty_structs_with_brackets, // structs without fields, with brackets
    clippy::enum_glob_use, // checks for `use Enum::*`
    clippy::semicolon_if_nothing_returned, // expression returns () not followed by a semicolon
    clippy::single_match_else, // matches with two arms where an if let else will usually suffice
    clippy::trivially_copy_pass_by_ref, // fns with ref args that could be passed by value
)]
#![cfg_attr(
    not(all(doc, feature = "_docsrs_stable")), // if docs are incomplete
    allow(rustdoc::broken_intra_doc_links) // allow broken intra-doc links
)]
#![allow(
    unknown_lints, // unrecognized lint attributes
    //stable_features, // a feature attribute that has since been made stable
    clippy::empty_docs, // empty documentation
    clippy::doc_lazy_continuation, // markdown lazy paragraph continuations
    clippy::mixed_attributes_style, // items with mixed (inner/outer) attributes
    clippy::module_inception, // modules with the same name as its parent
    clippy::wrong_self_convention, // `is_` methods having an owned self
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
        feature = "unsafe_layout", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// but you can enable `safe_*` features to prevent `unsafe` use in specific modules.

pub mod code;
pub mod data;
pub mod error;
pub mod exec;
pub mod mem;
pub mod num;
pub mod sys;
pub mod text;

/// All the library items.
pub mod all {
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::all::*,
        data::all::*,
        error::all::*,
        exec::all::*,
        mem::all::*,
        num::all::*,
        sys::all::*,
        text::all::*,
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
