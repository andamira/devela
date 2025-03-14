// devela::lib
//
//! A cohesive development layer.
//!
#![doc = include_str!("./Lib.md")]
//

/* global configuration */
//
// lints
//
// (Most lints are defined in Cargo.toml::lints)
// https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
//
// WAIT: [Per-crate-type lint configuration](https://github.com/rust-lang/cargo/issues/15046)
#![deny(rustdoc::missing_crate_level_docs, rustdoc::missing_debug_implementations)]
#![cfg_attr(
    not(all(doc, feature = "_docsrs")), // if features are incomplete…
    // not(all(doc, feature = "_docs", feature = "std")),
    allow(rustdoc::broken_intra_doc_links) // …allow broken intra-doc links
)]
//
// environment
//
#![cfg_attr(not(feature = "std"), no_std)]
// #![no_implicit_prelude] //?
//
// safety
//
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly (flags)
//
// (In sync with ../Cargo.toml::[lints.rust.unexpected_cfgs] & ../build/features.rs::FLAGS_NIGHTLY)
#![cfg_attr(nightly_allocator, feature(allocator_api))]
#![cfg_attr(nightly_autodiff, feature(autodiff))]
#![cfg_attr(nightly_bigint, feature(bigint_helper_methods))]
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
#![cfg_attr(nightly_float, feature(f16, f128))]
#![cfg_attr(nightly_simd, feature(portable_simd))]
// `nightly_stable` includes:
// ----------------------------
// `nightly_stable_next1`: 1.86 core, alloc, std…
#![cfg_attr(nightly_stable_next1, feature(
    const_black_box,
    const_is_char_boundary,
    float_next_up_down,
    get_many_mut, //  get_disjoint_mut
    non_zero_count_ones,
    target_feature_11,
    trait_upcasting,
))]
#![cfg_attr(all(nightly_stable_next1, feature = "alloc"), feature(vec_pop_if,))]
#![cfg_attr(all(nightly_stable_next1, feature = "std"), feature(const_mut_cursor, map_many_mut,))]
// ----------------------------
// `nightly_stable_next2`: 1.87 core, alloc, std…
#![cfg_attr(
    nightly_stable_next2,
    feature(
        const_copy_from_slice,
        const_slice_flatten,
        integer_sign_cast,
        num_midpoint_signed,
        const_ptr_sub_ptr,
        const_str_from_utf8,
        ptr_sub_ptr,
        slice_take,
        unbounded_shifts,
        unsigned_is_multiple_of,
    )
)]
#![cfg_attr(
    all(nightly_stable_next2, feature = "alloc"),
    feature(box_uninit_write, const_vec_string_slice, extract_if, string_extend_from_within,)
)]
#![cfg_attr(
    all(nightly_stable_next2, feature = "std"),
    feature(file_lock, hash_extract_if, os_str_display,)
)]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        asm_goto,
        assert_matches,
        cell_update,
        const_array_from_ref,
        const_cell,
        const_char_classify,
        const_slice_from_ref,
        const_sockaddr_setters,
        const_str_split_at,
        const_swap_nonoverlapping,
        c_str_module,
        derive_coerce_pointee,
        impl_trait_in_assoc_type,
        isqrt,
        let_chains,
        macro_metavar_expr,
        naked_functions,
        precise_capturing_in_traits,
        // repr128, // incomplete_features
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(all(nightly_stable_later, feature = "alloc"), feature(new_zeroed_alloc,))]
#![cfg_attr(
    all(nightly_stable_later, feature = "std"),
    feature(anonymous_pipe, once_wait, os_string_pathbuf_leak,)
)]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]

/* global safeguards */

// environment
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
// safety
#[cfg(all(
    feature = "safe",
    // In sync with ../Cargo.toml::unsafe & ../build/features.rs::UNSAFE
    any(feature = "unsafe", // includes all 11 specific purposes below:
        feature = "unsafe_array", feature = "unsafe_ffi", feature = "unsafe_hint",
        feature = "unsafe_layout", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str", feature = "unsafe_sync",
        feature = "unsafe_syscall", feature = "unsafe_thread",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// (note: you can enable `safe_*` features to prevent `unsafe` use in specific modules)

// https://doc.rust-lang.org/nightly/reference/names/preludes.html#extern-prelude
extern crate self as devela;

/* root modules */

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

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library.*
#[doc(inline)]
pub use ::core as _core;

pub mod _dep;
pub mod _info;

/* structural re-exports */

#[doc(hidden)]
pub use all::*;
pub mod all {
    // public items, feature-gated, visible at their origin and here in `all`
    //
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::_all::*,
        data::_all::*,
        lang::_all::*,
        media::_all::*,
        num::_all::*,
        phys::_all::*,
        sys::_all::*,
        text::_all::*,
        ui::_all::*,
        work::_all::*,
        //
        _always::*,
    };
}
mod _always {
    // public items, not as much feature-gated, bubbled up
    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub use super::{
        code::_always::*,
        data::_always::*,
        lang::_always::*,
        media::_always::*,
        num::_always::*,
        phys::_always::*,
        sys::_always::*,
        text::_always::*,
        // ui::_always::*,
        work::_always::*,
    };
}
#[doc(hidden)]
pub use _hidden::*;
mod _hidden {
    // public, hidden items
    pub use super::sys::_hidden::*;
}
#[allow(unused_imports)]
pub(crate) use _internals::*;
mod _internals {
    // private, internal items
    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub(crate) use super::{
        code::_internals::*,
        data::_internals::*,
        lang::_internals::*,
        num::_internals::*,
    };
}
