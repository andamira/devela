// devela::lib
//
//! A development layer in Rust.
//!
#![doc = include_str!("./Lib.md")]
//

/* crate configuration */
//
// lints
//
// (Most lints are defined in Cargo.toml::lints)
// https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
//
// WAIT: [Per-crate-type lint configuration](https://github.com/rust-lang/cargo/issues/15046)
#![cfg_attr(
    not(all(doc, feature = "_docs")), // if features are incomplete…
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
// (In sync with ../Cargo.toml::[workpace.lints.rust.unexpected_cfgs],
//  ../build/main/features.rs::FLAGS_NIGHTLY) && ../libs/base_core/src/lib.rs
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
// `nightly_stable_1_91`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_91,
    feature(
        array_repeat,
        as_array_of_cells,
        const_array_each_ref,
        const_type_id,
        duration_constructors_lite,
        ip_from,
        iter_chain,
        round_char_boundary,
        strict_overflow_ops,
        strict_provenance_atomic_ptr,
        unsigned_signed_diff,
        // unsigned_bigint_helpers, // (bigint_helper_methods),
    )
)]
// #![cfg_attr(all(nightly_stable_1_91, feature = "alloc"), feature())]
#![cfg_attr(
    all(nightly_stable_1_91, feature = "std"),
    feature(
        const_pathbuf_osstring_new,
        panic_payload_as_str,
        path_add_extension,
        path_file_prefix,
    )
)]
// ----------------------------
// `nightly_stable_1_92`: core, alloc, std…
#![cfg_attr(nightly_stable_1_92, feature())]
#![cfg_attr(
    all(nightly_stable_1_92, feature = "alloc"),
    feature(file_with_nul, btree_entry_insert,)
)]
#![cfg_attr(all(nightly_stable_1_92, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_1_93`: core, alloc, std…
// #![cfg_attr(nightly_stable_1_93, feature())]
// #![cfg_attr(all(nightly_stable_1_93, feature = "alloc"), feature())]
// #![cfg_attr(all(nightly_stable_1_93, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        assert_matches,
        breakpoint,
        cfg_select,
        cfg_version,
        const_array_from_ref,
        const_char_classify,
        const_slice_from_ref,
        const_slice_rotate,
        const_sockaddr_setters,
        const_str_split_at,
        debug_closure_helpers,
        derive_coerce_pointee,
        fn_align,
        if_let_guard,
        impl_trait_in_assoc_type,
        isqrt,
        macro_metavar_expr,
        macro_metavar_expr_concat,
        more_qualified_paths,
        offset_of_enum,
        offset_of_slice,
        substr_range,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(btree_extract_if, new_zeroed_alloc, vec_deque_pop_if,)
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "std"),
    feature(once_wait, rwlock_downgrade, stdarch_s390x_feature_detection,)
)]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]

/* crate safeguards */

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
#[cfg(feature = "std")]
extern crate std;

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

#[doc(hidden)]
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
}

// public, hidden items
#[doc(hidden)]
pub use _hidden::*;
mod _hidden {
    pub use super::sys::_hidden::*;
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
mod _crate_internals {
    pub(crate) use devela_base_core::_workspace_internals::*;
    pub(crate) use devela_base_num::_workspace_internals::*;

    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        lang::_crate_internals::*,
        media::_crate_internals::*,
        num::_crate_internals::*,
    };
}
