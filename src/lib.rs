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
// doc:
// https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
// #![doc(test(attr(warn(dead_code))))]
#![cfg_attr(
    not(all(doc, feature = "_docsrs_stable")), // if docs are incomplete...
    allow(rustdoc::broken_intra_doc_links) // …allow broken intra-doc links
)]
//
// environment:
#![cfg_attr(not(feature = "std"), no_std)]
//
// safety:
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly:
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(feature = "nightly_doc", miri), allow(unused_attributes))]
#![cfg_attr(all(feature = "nightly_doc", not(doc)), allow(unused_attributes))]
#![cfg_attr(feature = "nightly_coro", feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(feature = "nightly_simd", feature(portable_simd))]
// "nightly_stable_soon" includes:
#![cfg_attr( // 1.83
    feature = "nightly_stable_next1",
    feature(
        const_cell_into_inner,
        const_char_encode_utf8,
        const_extern_fn,
        const_float_bits_conv,
        const_float_classify,
        const_intrinsic_copy,
        const_maybe_uninit_as_mut_ptr,
        const_mut_refs,
        const_refs_to_cell,
        const_refs_to_static,
        const_slice_from_raw_parts_mut,
        const_slice_split_at_mut,
        duration_consts_float,
        waker_getters,
    )
)]
#![cfg_attr( // 1.83 not(miri)
    all(not(miri), feature = "nightly_stable_next1"),
    feature(
        io_error_more,
        entry_insert,
        mpmc_channel,
    )
)]
#![cfg_attr( // 1.84
    feature = "nightly_stable_next1",
    feature(
        const_make_ascii,
        const_maybe_uninit_assume_init,
        pin_deref_mut,
    )
)]
// #![cfg_attr( // 1.84 not(miri)
//     all(not(miri), feature = "nightly_stable_next1"),
//     feature(
//     )
// )]
#![cfg_attr( // 1.??
    feature = "nightly_stable",
    feature(
        async_closure,
        const_array_from_ref,
        const_atomic_from_ptr,
        const_maybe_uninit_write,
        const_slice_from_ref,
        impl_trait_in_assoc_type,
        isqrt,
        let_chains,
        macro_metavar_expr,
        noop_waker,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr( // 1.?? not(miri)
    all(not(miri), feature = "nightly_stable"),
    feature(
        box_uninit_write,
        new_zeroed_alloc,
    )
)]

// safeguard environment:
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
//
// safeguard safety:
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", // includes all 12 specific purposes below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_const",
        feature = "unsafe_hint", feature = "unsafe_layout", feature = "unsafe_niche",
        feature = "unsafe_ptr", feature = "unsafe_slice", feature = "unsafe_str",
        feature = "unsafe_sync", feature = "unsafe_syscall", feature = "unsafe_thread",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// (note: you can enable `safe_*` features to prevent `unsafe` use in specific modules)

//* root modules *//

extern crate self as devela;

pub mod code;
pub mod data;
pub mod error;
pub mod mem;
pub mod num;
pub mod rend;
pub mod sys;
pub mod text;
pub mod work;

/// All the crate's items re-exported flat.
/// <br/><hr>
///
/// There's a more exhaustive list that includes all of the dependencies items,
/// without descriptions, in [All items](../all.html).
pub mod all {
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::all::*,
        data::all::*,
        error::all::*,
        mem::all::*,
        num::all::*,
        rend::all::*,
        sys::all::*,
        text::all::*,
        work::all::*,
    };
}
#[doc(no_inline)]
#[allow(unused_imports)]
pub use all::*;

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library.*
#[doc(inline)]
pub use ::core as _core;

/// Re-exported optional dependencies.
pub mod _dep;

/// Documentation about the library.
#[cfg(any(doc, test))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(doc)))]
pub mod _doc {
    /// Documented examples.
    pub mod examples;

    /// Library features explained.
    #[cfg(doc)]
    pub mod features {
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./_doc/features.md")]
    }
}
