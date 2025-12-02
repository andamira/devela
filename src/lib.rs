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
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))] // configured below
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
#![cfg_attr(nightly_stable_1_92, feature(const_slice_rotate,))]
#![cfg_attr(
    all(nightly_stable_1_92, feature = "alloc"),
    feature(file_with_nul, btree_entry_insert,)
)]
#![cfg_attr(all(nightly_stable_1_92, feature = "std"), feature(rwlock_downgrade,))]
// ----------------------------
// `nightly_stable_1_93`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_93,
    feature(
        asm_cfg,
        char_max_len,
        core_slice_as_array,
        duration_from_nanos_u128,
        fmt_from_fn,
        maybe_uninit_slice,
        maybe_uninit_write_slice,
        unchecked_neg,
        unchecked_shifts,
    )
)]
#![cfg_attr(
    all(nightly_stable_1_93, feature = "alloc"),
    feature(vec_deque_pop_if, vec_into_raw_parts)
)]
#![cfg_attr(all(nightly_stable_1_93, feature = "std"), feature(stdarch_s390x_feature_detection,))]
// ----------------------------
// `nightly_stable_1_94`: core, alloc, std…
// #![cfg_attr(nightly_stable_1_94, feature())]
// #![cfg_attr(all(nightly_stable_1_94, feature = "alloc"), feature())]
// #![cfg_attr(all(nightly_stable_1_94, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        alloc_layout_extra,
        array_windows,
        assert_matches,
        atomic_try_update,
        breakpoint,
        cfg_select,
        cfg_version,
        const_array_from_ref,
        const_char_classify,
        const_mul_add,
        const_slice_from_ref,
        const_sockaddr_setters,
        const_str_split_at,
        debug_closure_helpers,
        derive_coerce_pointee,
        fn_align,
        frontmatter,
        if_let_guard,
        impl_trait_in_assoc_type,
        isqrt,
        macro_metavar_expr,
        macro_metavar_expr_concat,
        more_qualified_paths,
        offset_of_enum,
        offset_of_slice,
        peekable_next_if_map,
        substr_range,
        supertrait_item_shadowing,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(btree_extract_if, new_zeroed_alloc,)
)]
#![cfg_attr(all(nightly_stable_later, feature = "std"), feature(once_wait,))]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]
//
// documentation
//
// #![cfg_attr(nightly_doc, doc(auto_cfg = false))]
#![cfg_attr(nightly_doc, doc(auto_cfg(hide( // do not document:
    // development features
    feature = "__dbg", feature = "__std", feature = "__publish",
    // positive safety features
    feature = "safe", feature = "safe_build", feature = "safe_code", feature = "safe_data",
    feature = "safe_lang", feature = "safe_media", feature = "safe_audio", feature = "safe_color",
    feature = "safe_draw", feature = "safe_font", feature = "safe_image", feature = "safe_game",
    feature = "safe_num", feature = "safe_phys", feature = "safe_time", feature = "safe_sys",
    feature = "safe_io", feature = "safe_mem", feature = "safe_text", feature = "safe_ui",
    feature = "safe_layout", feature = "safe_work",
))))]

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

pub mod _doc;

/* structural re-exports */

#[doc(hidden)]
pub use zall_ as all_;
#[rustfmt::skip]
#[doc = crate::_DOC_ZALL_!()]
pub mod zall_ {
    crate::CONST! { COMMON_DOC = "\n\nAll root module's items flat re-exported."; }
    #[doc = concat![crate::_DOC_CODE!(), COMMON_DOC!()]]
    pub mod _code { #[allow(unused)] pub use super::super::code::_all::*; }
    #[doc = concat![crate::_DOC_DATA!(), COMMON_DOC!()]]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }
    #[doc = concat![crate::_DOC_GAME!(), COMMON_DOC!()]]
    pub mod _game { #[allow(unused)] pub use super::super::game::_all::*; }
    #[doc = concat![crate::_DOC_LANG!(), COMMON_DOC!()]]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }
    #[doc = concat![crate::_DOC_MEDIA!(), COMMON_DOC!()]]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }
    #[doc = concat![crate::_DOC_NUM!(), COMMON_DOC!()]]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }
    #[doc = concat![crate::_DOC_PHYS!(), COMMON_DOC!()]]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }
    #[doc = concat![crate::_DOC_SYS!(), COMMON_DOC!()]]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }
    #[doc = concat![crate::_DOC_TEXT!(), COMMON_DOC!()]]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }
    #[doc = concat![crate::_DOC_UI!(), COMMON_DOC!()]]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }
    #[doc = concat![crate::_DOC_WORK!(), COMMON_DOC!()]]
    /// <br/><hr>
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

// public items, feature-gated, visible at their origin and in `zall`:
// WAIT: [doc(canonica)](https://github.com/rust-lang/rfcs/issues/3011)
#[doc(hidden)]
pub use {zall as all, zall::*}; // keep devela::all::* accesor hidden
#[doc = crate::_DOC_ZALL!()]
pub mod zall {
    #[doc(inline)]
    #[rustfmt::skip]
    #[allow(unused_imports)]
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
    #[rustfmt::skip]
    #[allow(unused_imports)]
    pub use super::{
        media::_hidden::*,
        sys::_hidden::*,
    };
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
mod _crate_internals {
    #![allow(unused_imports)]

    pub(crate) use devela_base_core::_workspace_internals::*;
    // #[cfg(feature = "devela_base_data")]
    // pub(crate) use devela_base_data::_workspace_internals::*;
    pub(crate) use devela_base_num::_workspace_internals::*;
    #[cfg(feature = "devela_base_text")]
    pub(crate) use devela_base_text::_workspace_internals::*;

    #[rustfmt::skip]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        lang::_crate_internals::*,
        num::_crate_internals::*,
    };
}
