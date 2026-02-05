// devela::index
//
//! A development layer in Rust.
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
// environment
#![cfg_attr(not(feature = "std"), no_std)]
// #![no_implicit_prelude] //?
//
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly (flags)
// ```
// RUSTFLAGS="--cfg nightly_stable" cargo +nightly build
// RUSTDOCFLAGS="--cfg nightly_stable" cargo +nightly doc
// ```
// (In sync with ../Cargo.toml::[workpace.lints.rust.unexpected_cfgs],
//  ../build/main/features.rs::FLAGS_NIGHTLY) && ../src/base/core/src/index.rs
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))] // configured below
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
//
#![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_float, feature(f16, f128))]
#![cfg_attr(nightly_simd, feature(portable_simd))]
//
// `nightly_stable` includes:
// ----------------------------
// `nightly_stable_1_94`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_94,
    feature(
        alloc_layout_extra,
        array_windows,
        euler_gamma_golden_ratio,
        lazy_get,
        peekable_next_if_map,
    )
)]
// #![cfg_attr(all(nightly_stable_1_94, feature = "alloc"), feature())]
#![cfg_attr(all(nightly_stable_1_94, feature = "std"), feature(const_mul_add))]
// ----------------------------
// `nightly_stable_1_95`: core, alloc, std…
// #![cfg_attr(nightly_stable_1_95, feature())]
#![cfg_attr(all(nightly_stable_1_95, feature = "alloc"), feature(push_mut,))]
// #![cfg_attr(all(nightly_stable_1_95, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_1_96`: core, alloc, std…
// #![cfg_attr(nightly_stable_1_96, feature())]
// #![cfg_attr(all(nightly_stable_1_96, feature = "alloc"), feature())]
// #![cfg_attr(all(nightly_stable_1_96, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        assert_matches,
        atomic_try_update,
        breakpoint,
        cfg_select,
        cfg_version,
        cold_path,
        const_array_from_ref,
        const_char_classify,
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
        str_as_str,
        substr_range,
        supertrait_item_shadowing,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(btree_extract_if, new_zeroed_alloc, vec_deque_truncate_front,)
)]
#![cfg_attr(all(nightly_stable_later, feature = "std"), feature(once_wait,))]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]
//
// documentation
//
// #![cfg_attr(nightly_doc, doc(auto_cfg = false))]
// NOTE: that doc attribute hiding only works in nightly
#![cfg_attr(nightly_doc, doc(auto_cfg(hide( // do not document:
    miri,
    // development features
    feature = "__dbg", feature = "__std", feature = "__publish",
    // positive safety features
    feature = "safe", feature = "safe_build", feature = "safe_code", feature = "safe_data",
    feature = "safe_geom", feature = "safe_lang", feature = "safe_media", feature = "safe_audio",
    feature = "safe_color", feature = "safe_draw", feature = "safe_font", feature = "safe_image",
    feature = "safe_num", feature = "safe_org", feature = "safe_phys", feature = "safe_time",
    feature = "safe_run", feature = "safe_sys", feature = "safe_io", feature = "safe_mem",
    feature = "safe_text", feature = "safe_ui", feature = "safe_vita", feature = "safe_work",
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
macro_rules! __crate_name {
    () => {
        "devela"
    };
}
pub(crate) use __crate_name;

/* root modules */

pub mod code;
pub mod data;
pub mod geom;
pub mod lang;
pub mod media;
pub mod num;
pub mod org;
pub mod phys;
pub mod run;
pub mod sys;
pub mod text;
pub mod ui;
pub mod vita;
pub mod work;
// internal:
pub mod yard;
#[doc(hidden)]
pub use yard::_dep;

pub mod _doc;

/* structural re-exports */

#[doc(hidden)]
pub use zall_ as all_;
#[rustfmt::skip]
#[doc = crate::_DOC_ZALL_!()]
pub mod zall_ {
    macro_rules! COMMON_DOC { ($mod:literal) => { concat!(" ",
        crate::_doc!(root:$mod), "\n\nAll `", $mod,"` module's items flat re-exported.") }; }
    #[doc = concat![crate::_DOC_CODE!(), crate::_DOC_CODE_MODULES!(), COMMON_DOC!("code")]]
    pub mod _code { #[allow(unused)] pub use super::super::code::_all::*; }
    #[doc = concat![crate::_DOC_DATA!(), crate::_DOC_DATA_MODULES!(), COMMON_DOC!("data")]]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }
    #[doc = concat![crate::_DOC_GEOM!(), crate::_DOC_GEOM_MODULES!(), COMMON_DOC!("geom")]]
    pub mod _geom { #[allow(unused)] pub use super::super::geom::_all::*; }
    #[doc = concat![crate::_DOC_LANG!(), crate::_DOC_LANG_MODULES!(), COMMON_DOC!("lang")]]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }
    #[doc = concat![crate::_DOC_MEDIA!(), crate::_DOC_MEDIA_MODULES!(), COMMON_DOC!("media")]]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }
    #[doc = concat![crate::_DOC_NUM!(), crate::_DOC_NUM_MODULES!(), COMMON_DOC!("num")]]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }
    #[doc = concat![crate::_tags!(wip),
        crate::_DOC_ORG!(), crate::_DOC_ORG_MODULES!(), COMMON_DOC!("org")]]
    pub mod _org { #[allow(unused)] pub use super::super::org::_all::*; }
    #[doc = concat![crate::_DOC_PHYS!(), crate::_DOC_PHYS_MODULES!(), COMMON_DOC!("phys")]]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }
    #[doc = concat![crate::_DOC_RUN!(), crate::_DOC_RUN_MODULES!(), COMMON_DOC!("run")]]
    pub mod _run { #[allow(unused)] pub use super::super::run::_all::*; }
    #[doc = concat![crate::_DOC_SYS!(), crate::_DOC_SYS_MODULES!(), COMMON_DOC!("sys")]]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }
    #[doc = concat![crate::_DOC_TEXT!(), crate::_DOC_TEXT_MODULES!(), COMMON_DOC!("text")]]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }
    #[doc = concat![crate::_DOC_UI!(), crate::_DOC_UI_MODULES!(), COMMON_DOC!("ui")]]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }
    #[doc = concat![crate::_tags!(wip),
        crate::_DOC_VITA!(), crate::_DOC_VITA_MODULES!(), COMMON_DOC!("vita")]]
    pub mod _vita { #[allow(unused)] pub use super::super::vita::_all::*; }
    #[doc = concat![crate::_DOC_WORK!(), crate::_DOC_WORK_MODULES!(), COMMON_DOC!("work")]]
    /// <br/><hr>
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

// public items, feature-gated, visible at their origin and in `zall`:
// WAIT: [doc(canonica)](https://github.com/rust-lang/rfcs/issues/3011)
#[doc(hidden)]
pub use {zall as all, zall::*}; // keep devela::all::* accesor hidden
#[doc = crate::_DOC_ZALL!()]
#[rustfmt::skip]
pub mod zall {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        code::_all::*,
        data::_all::*,
        geom::_all::*,
        lang::_all::*,
        media::_all::*,
        num::_all::*,
        org::_all::*,
        phys::_all::*,
        run::_all::*,
        sys::_all::*,
        text::_all::*,
        ui::_all::*,
        vita::_all::*,
        work::_all::*,
    };
}

// public, hidden items
#[doc(hidden)]
pub use _hidden::*;
#[rustfmt::skip]
mod _hidden {
    #![allow(unused_imports)]

    pub use devela_base_core::_hidden::*;
    pub use super::{
        media::_hidden::*,
        num::_hidden::*,
        sys::_hidden::*,
    };
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
// NOTE: module-level `#[rustfmt::skip]` breaks macro import resolution here.
mod _crate_internals {
    #![allow(unused_imports)]

    pub(crate) use devela_base_core::_workspace_internals::*;

    #[rustfmt::skip]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        geom::_crate_internals::*,
        lang::_crate_internals::*,
        media::_crate_internals::*,
        num::_crate_internals::*,
        org::_crate_internals::*,
        phys::_crate_internals::*,
        run::_crate_internals::*,
        sys::_crate_internals::*,
        text::_crate_internals::*,
        ui::_crate_internals::*,
        vita::_crate_internals::*,
        work::_crate_internals::*,
    };
}
