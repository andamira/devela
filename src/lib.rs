// devela::lib
//
//! A cohesive development layer.
//!
#![doc = include_str!("./Lib.md")]
//

//* global config *//
//
// Most lints are set in Cargo.toml
//
// docs
// https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
// #![doc(test(attr(warn(dead_code))))]
#![cfg_attr(
    not(all(doc, feature = "_docsrs_stable")), // if docs are incomplete...
    allow(rustdoc::broken_intra_doc_links) // …allow broken intra-doc links
)]
//
// environment
// #![no_implicit_prelude] // MAYBE
#![cfg_attr(not(feature = "std"), no_std)]
//
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly
// In sync with Cargo.toml::nightly & build/features.rs::NIGHTLY
#![cfg_attr(feature = "nightly_autodiff", feature(autodiff))]
#![cfg_attr(feature = "nightly_coro", feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(feature = "nightly_doc", miri), allow(unused_attributes))]
#![cfg_attr(all(feature = "nightly_doc", not(doc)), allow(unused_attributes))]
#![cfg_attr(feature = "nightly_float", feature(f16, f128))]
#![cfg_attr(feature = "nightly_simd", feature(portable_simd))]
// "nightly_stable" includes:
#![cfg_attr( // 1.84
    feature = "nightly_stable_next1",
    feature(
        const_atomic_from_ptr,
        const_char_encode_utf16,
        const_make_ascii,
        const_maybe_uninit_assume_init,
        const_option_ext,
        const_pin, // subset (_2)
        const_ptr_is_null,
        const_unicode_case_lookup,
        // ip, // subset
        pin_deref_mut,
        result_ffi_guarantees,
    )
)]
// #![cfg_attr( // 1.84 not(miri)
//     all(not(miri), feature = "nightly_stable_next1"),
//     feature()
// )]
#![cfg_attr( // 1.85
    feature = "nightly_stable_next2",
    feature(
        build_hasher_default_const_new,
        const_align_of_val,
        // const_collections_with_hasher, // ?
        const_float_methods,
        const_maybe_uninit_write,
        const_size_of_val,
        extended_varargs_abi_support,
        num_midpoint,
        ptr_fn_addr_eq,
    )
)]
// #![cfg_attr( // 1.85 not(miri)
//     all(not(miri), feature = "nightly_stable_next2"),
//     feature()
// )]
#![cfg_attr( // 1.??
    feature = "nightly_stable_later",
    feature(
        asm_goto,
        async_closure,
        // box_uninit_write, // ?
        const_array_from_ref,
        const_slice_from_ref,
        // derive_smart_pointer, // x
        do_not_recommend,
        impl_trait_in_assoc_type,
        isqrt,
        let_chains,
        macro_metavar_expr,
        // new_zeroed_alloc, // ?
        noop_waker,
        unbounded_shifts,
        unsafe_cell_from_mut,
    )
)]
// #![cfg_attr( // 1.?? not(miri)
//     all(not(miri), feature = "nightly_stable_later"),
//     feature()
// )]

// environment safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
//
// safety safeguards
#[cfg(all(
    feature = "safe",
    // In sync with Cargo.toml::unsafe & build/features.rs::SAFETY
    any(feature = "unsafe", // includes all 12 specific purposes below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_hint",
        feature = "unsafe_layout", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str", feature = "unsafe_sync",
        feature = "unsafe_syscall", feature = "unsafe_thread",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// (note: you can enable `safe_*` features to prevent `unsafe` use in specific modules)

//* root modules *//

extern crate self as devela;

pub mod code;
pub mod data;
pub mod lang;
pub mod media;
pub mod num;
pub mod phys;
pub mod sys;
pub mod text;
pub mod ui;
pub mod work;

// public items, visible in several places
#[doc(hidden)]
pub use all::*;
pub mod all {
    //! All the crate's items re-exported flat.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module from its parent.
    //!
    //! There's a more exhaustive list that includes all of the dependencies items,
    //! without their descriptions, in [All items](../all.html).
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::all::*,
        data::all::*,
        lang::all::*,
        media::all::*,
        num::all::*,
        phys::all::*,
        sys::all::*,
        text::all::*,
        ui::all::*,
        work::all::*,
    };
}
// public items, hidden everywhere
#[doc(hidden)]
pub use items_hidden::*;
mod items_hidden {
    pub use super::sys::items_hidden::*;
}
// private items, crate-use only
pub(crate) use items_private::*;
mod items_private {
    pub(crate) use super::code::items_private::*;
}

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library.*
#[doc(inline)]
pub use ::core as _core;

pub mod _dep;
pub mod _doc;
