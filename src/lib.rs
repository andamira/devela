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
#![cfg_attr(
    not(all(doc, feature = "_docsrs_stable")), // if features are incomplete…
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
// nightly
//
// (In sync with Cargo.toml::nightly & build/features.rs::NIGHTLY)
#![cfg_attr(feature = "nightly_allocator", feature(allocator_api))]
#![cfg_attr(feature = "nightly_autodiff", feature(autodiff))]
#![cfg_attr(feature = "nightly_bigint", feature(bigint_helper_methods))]
#![cfg_attr(feature = "nightly_coro", feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(feature = "nightly_doc", miri), allow(unused_attributes))]
#![cfg_attr(all(feature = "nightly_doc", not(doc)), allow(unused_attributes))]
#![cfg_attr(feature = "nightly_float", feature(f16, f128))]
#![cfg_attr(feature = "nightly_simd", feature(portable_simd))]
// "nightly_stable" includes:
// "nightly_stable_next1": 1.85 core, alloc, std…
#![cfg_attr(feature = "nightly_stable_next1", feature(
    async_closure,
    build_hasher_default_const_new,
    const_align_of_val,
    const_maybe_uninit_write,
    const_nonnull_new,
    const_size_of_val,
    const_swap,
    coverage_attribute,
    do_not_recommend, // diagnostics
    extended_varargs_abi_support,
    noop_waker,
    num_midpoint,
    ptr_fn_addr_eq,
))]
#![cfg_attr(all(feature = "nightly_stable_next1", feature = "alloc"), feature())]
#![cfg_attr(
    all(feature = "nightly_stable_next1", feature = "std"),
    feature(const_collections_with_hasher,)
)]
// "nightly_stable_next2": 1.86 core, alloc, std…
#![cfg_attr(feature = "nightly_stable_next2", feature(float_next_up_down,))]
#![cfg_attr(all(feature = "nightly_stable_next2", feature = "alloc"), feature())]
#![cfg_attr(all(feature = "nightly_stable_next2", feature = "std"), feature())]
// "nightly_stable_later": 1.?? core, alloc, std, not(miri)…
#![cfg_attr(feature = "nightly_stable_later", feature(
    // anonymous_pipe, // ✗
    asm_goto,
    cell_update,
    const_array_from_ref,
    const_black_box,
    const_is_char_boundary,
    const_slice_flatten,
    const_slice_from_ref,
    const_str_split_at,
    derive_coerce_pointee,
    get_many_mut, // get_disjoint_mut (new name)
    impl_trait_in_assoc_type,
    isqrt,
    let_chains,
    macro_metavar_expr,
    naked_functions,
    num_midpoint_signed,
    trait_upcasting,
    unbounded_shifts,
    unsafe_cell_from_mut,
))]
#![cfg_attr(
    all(feature = "nightly_stable_later", feature = "alloc"),
    feature(box_uninit_write, new_zeroed_alloc, vec_pop_if,)
)]
#![cfg_attr(all(feature = "nightly_stable_later", feature = "std"), feature(hash_extract_if,))]
#![cfg_attr(all(feature = "nightly_stable_later", not(miri)), feature())]

/* global safeguards */

// environment
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
// safety
#[cfg(all(
    feature = "safe",
    // In sync with Cargo.toml::unsafe & build/features.rs::UNSAFE
    any(feature = "unsafe", // includes all 11 specific purposes below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_hint",
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
    //! All the crate's items re-exported flat.
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
        ui::_always::*,
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
        num::_internals::*,
    };
}
