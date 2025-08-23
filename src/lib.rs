// devela::lib
//
//! A development layer in Rust.
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
// ```
// RUSTFLAGS="--cfg nightly_stable" cargo +nightly build
// RUSTDOCFLAGS="--cfg nightly_stable" cargo +nightly doc
// ```
//
// (In sync with ../Cargo.toml::[workpace.lints.rust.unexpected_cfgs] &&
//  ../build/main/features.rs::FLAGS_NIGHTLY)
#![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
#![cfg_attr(nightly_bigint, feature(bigint_helper_methods))]
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
#![cfg_attr(nightly_float, feature(f16, f128))]
#![cfg_attr(nightly_simd, feature(portable_simd))]
//
// `nightly_stable` includes:
// ----------------------------
// `nightly_stable_1_90`: core, alloc, std…
#![cfg_attr(nightly_stable_1_90, feature(const_slice_reverse, mixed_integer_ops_unsigned_sub,))]
// #![cfg_attr(all(nightly_stable_1_90, feature = "alloc"), feature())]
#![cfg_attr(all(nightly_stable_1_90, feature = "std"), feature(const_float_round_methods,))]
// ----------------------------
// `nightly_stable_1_91`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_91,
    feature(
        as_array_of_cells,
        const_array_each_ref,
        const_type_id,
        duration_constructors_lite,
        ip_from,
        iter_chain,
        strict_overflow_ops,
        unsigned_signed_diff,
    )
)]
// #![cfg_attr(all(nightly_stable_1_91, feature = "alloc"), feature())]
#![cfg_attr(
    all(nightly_stable_1_91, feature = "std"),
    feature(const_pathbuf_osstring_new, panic_payload_as_str, path_file_prefix,)
)]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        array_repeat,
        assert_matches,
        breakpoint,
        cfg_select,
        cfg_version,
        const_array_from_ref,
        const_char_classify,
        const_slice_from_ref,
        const_sockaddr_setters,
        const_str_split_at,
        derive_coerce_pointee,
        fn_align,
        if_let_guard,
        impl_trait_in_assoc_type,
        isqrt,
        macro_metavar_expr,
        more_qualified_paths,
        offset_of_enum,
        offset_of_slice,
        substr_range,
        strict_provenance_atomic_ptr,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(btree_entry_insert, btree_extract_if, new_zeroed_alloc,)
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "std"),
    feature(
        file_with_nul,
        once_wait,
        path_add_extension,
        rwlock_downgrade,
        stdarch_s390x_feature_detection,
    )
)]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]

/* global safeguards */

// environment
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
// safety
#[cfg(all(
    feature = "safe",
    // In sync with ../Cargo.toml::unsafe & ../build/main/features.rs::UNSAFE
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
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate self as devela;

/* root modules */

pub mod code;
pub mod data;
pub mod game;
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

// public items, feature-gated, visible at their origin and in `all`:
#[doc(hidden)]
pub use all::*;
#[rustfmt::skip]
pub mod all { #![allow(unused_imports)]
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[doc(inline)]
    pub use super::{
        code::_all::*,
        data::_all::*,
        game::_all::*,
        lang::_all::*,
        media::_all::*,
        num::_all::*,
        phys::_all::*,
        sys::_all::*,
        text::_all::*,
        ui::_all::*,
        work::_all::*,
    };
    // less feature-gated public items, bubbled up
    #[doc(inline)]
    pub use super::{
        code::_always::*,
        data::_always::*,
        lang::_always::*,
        // game::_always::*,
        media::_always::*,
        num::_always::*,
        phys::_always::*,
        sys::_always::*,
        text::_always::*,
        // ui::_always::*,
        work::_always::*,
    };
}

// public, hidden items
#[doc(hidden)]
pub use _hidden::*;
mod _hidden {
    pub use super::sys::_hidden::*;
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _internals::*;
mod _internals {
    pub(crate) use devela_base::_workspace_private::*;

    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub(crate) use super::{
        code::_internals::*,
        data::_internals::*,
        lang::_internals::*,
        media::_internals::*,
        num::_internals::*,
        sys::_internals::*,
    };
}
