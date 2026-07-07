// devela/src/index.rs
//
//! A development substrate of coherence.
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
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))] // enable for all doctests
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
// `nightly_stable_1_97`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_97,
    feature(
        int_lowest_highest_one,
        isolate_most_least_significant_one,
        uint_bit_width,
        cfg_target_has_atomic_equal_alignment,
    )
)]
// #![cfg_attr(all(nightly_stable_1_97, feature = "alloc"), feature())]
// #![cfg_attr(all(nightly_stable_1_97, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_1_98`: core, alloc, std…
#![cfg_attr(
    nightly_stable_1_98,
    feature(
        atomic_from_mut,
        float_algebraic,
        int_format_into,
        nonzero_from_str_radix,
        result_option_map_or_default,
        strip_circumfix,
        substr_range,
    )
)]
#![cfg_attr(
    all(nightly_stable_1_98, feature = "alloc"),
    feature(box_as_ptr, str_from_utf16_endian,)
)]
// #![cfg_attr(all(nightly_stable_1_98, feature = "std"), feature())]
// ----------------------------
// `nightly_stable_1_99`: core, alloc, std…
// #![cfg_attr(nightly_stable_1_99, feature())]
#![cfg_attr(all(nightly_stable_1_99, feature = "alloc"), feature(vec_deque_truncate_front,))]
#![cfg_attr(all(nightly_stable_1_99, feature = "std"), feature(local_key_cell_update,))]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(
    nightly_stable_later,
    feature(
        bool_to_result,
        breakpoint,
        cfg_version,
        const_array_from_ref,
        const_char_classify,
        const_slice_from_ref,
        const_sockaddr_setters,
        const_str_split_at,
        debug_closure_helpers,
        derive_coerce_pointee,
        exclusive_wrapper,
        float_bits_const,
        fn_align,
        frontmatter,
        impl_trait_in_assoc_type,
        isqrt,
        layout_for_ptr,
        likely_unlikely,
        macro_metavar_expr,
        macro_metavar_expr_concat,
        more_qualified_paths,
        new_range_api,
        offset_of_enum,
        offset_of_slice,
        optimize_attribute,
        random, //random_source,
        refcell_try_map,
        str_as_str,
        supertrait_item_shadowing,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(box_vec_non_null, btree_extract_if, new_zeroed_alloc,)
)]
#![cfg_attr(all(nightly_stable_later, feature = "std"), feature(once_wait, path_is_empty,))]
// #![cfg_attr(all(nightly_stable_later, not(miri)), feature())]
//
// documentation
//
// #![cfg_attr(nightly_doc, doc(auto_cfg = false))]
// NOTE: that doc attribute hiding only works in nightly
#![cfg_attr(nightly_doc, doc(auto_cfg(
    // rustc flags:
    hide(miri, ffi··, ffi_alsa··, ffi_xcb_shm··),
    // features:
    hide(feature, values(
        // development
        "__dbg", "__std", "__publish",
        // scope
        "_docs_examples", "_linux_abi",
        // positive safety
        "safe", "safe_build", "safe_code", "safe_data",
        "safe_geom", "safe_lang", "safe_media", "safe_audio",
        "safe_color", "safe_draw", "safe_font", "safe_image",
        "safe_num", "safe_org", "safe_phys", "safe_time",
        "safe_run", "safe_sys", "safe_io", "safe_mem",
        "safe_text", "safe_ui", "safe_vita", "safe_work",
    )),
)))]

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

pub mod code; // Code structure, generation, and reflective machinery.
pub mod data; // Stored values, access patterns, and structural organization.
pub mod error; // Explicit fallibility and recovery semantics.
pub mod geom; // Spatial forms, relations, and transformations.
pub mod lang; // Meaning, grammar, expression, and interpretation.
pub mod media; // Perceivable artifacts, formats, and synthesis.
pub mod num; // Magnitude, order, uncertainty, and formal computation.
pub mod org; // Collective structures and supra-individual agency.
pub mod phys; // Measured reality, units, matter, energy, and time.
pub mod run; // Runtime staging, progression, and application state.
pub mod sys; // Host systems, devices, memory, I/O, and OS boundaries.
pub mod text; // Symbolic sequences, encodings, and text processing.
pub mod ui; // Interaction, presentation state, and semantic controls.
pub mod vita; // Lived practices, capacities, embodiment, and play.
pub mod work; // Tasks, execution loci, scheduling, and synchronization.
// internal:
pub mod yard; // Scaffolding, taxonomy, and documentation support.

#[doc(hidden)]
pub use yard::dep as _dep;

pub mod _doc;

/* structural re-exports */

#[rustfmt::skip]
#[doc = crate::_DOC_ALL_!()]
#[doc = crate::_doc!(br+hr)] // gives way to the first root module
#[doc = crate::_DOC_ALL_PLUS_!()]
pub mod all_ {
    macro_rules! _COMMON_DOC { ($mod:literal) => { concat!(" ",
        crate::_doc!(root:$mod), "\n\nAll `", $mod,"` module's items flat re-exported.") }; }
    #[doc = concat![crate::_DOC_CODE!(), crate::_DOC_CODE_MODULES!(), _COMMON_DOC!("code")]]
    pub mod _code { #[allow(unused)] pub use super::super::code::_all::*; }
    #[doc = concat![crate::_DOC_DATA!(), crate::_DOC_DATA_MODULES!(), _COMMON_DOC!("data")]]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }
    #[doc = concat![crate::_DOC_ERROR!(), crate::_DOC_ERROR_MODULES!(), _COMMON_DOC!("error")]]
    pub mod _error { #[allow(unused)] pub use super::super::error::_all::*; }
    #[doc = concat![crate::_DOC_GEOM!(), crate::_DOC_GEOM_MODULES!(), _COMMON_DOC!("geom")]]
    pub mod _geom { #[allow(unused)] pub use super::super::geom::_all::*; }
    #[doc = concat![crate::_DOC_LANG!(), crate::_DOC_LANG_MODULES!(), _COMMON_DOC!("lang")]]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }
    #[doc = concat![crate::_DOC_MEDIA!(), crate::_DOC_MEDIA_MODULES!(), _COMMON_DOC!("media")]]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }
    #[doc = concat![crate::_DOC_NUM!(), crate::_DOC_NUM_MODULES!(), _COMMON_DOC!("num")]]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }
    #[doc = crate::_tags!(wip)]
    #[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
    #[doc = concat![crate::_DOC_ORG!(), crate::_DOC_ORG_MODULES!(), _COMMON_DOC!("org")]]
    pub mod _org { #[allow(unused)] pub use super::super::org::_all::*; }
    #[doc = concat![crate::_DOC_PHYS!(), crate::_DOC_PHYS_MODULES!(), _COMMON_DOC!("phys")]]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }
    #[doc = concat![crate::_DOC_RUN!(), crate::_DOC_RUN_MODULES!(), _COMMON_DOC!("run")]]
    pub mod _run { #[allow(unused)] pub use super::super::run::_all::*; }
    #[doc = concat![crate::_DOC_SYS!(), crate::_DOC_SYS_MODULES!(), _COMMON_DOC!("sys")]]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }
    #[doc = concat![crate::_DOC_TEXT!(), crate::_DOC_TEXT_MODULES!(), _COMMON_DOC!("text")]]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }
    #[doc = concat![crate::_DOC_UI!(), crate::_DOC_UI_MODULES!(), _COMMON_DOC!("ui")]]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }
    #[doc = concat![crate::_DOC_VITA!(), crate::_DOC_VITA_MODULES!(), _COMMON_DOC!("vita")]]
    pub mod _vita { #[allow(unused)] pub use super::super::vita::_all::*; }
    #[doc = concat![crate::_DOC_WORK!(), crate::_DOC_WORK_MODULES!(), _COMMON_DOC!("work")]]
    /// <br/><hr>
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

// public items, feature-gated, visible at their origin and in `all`:
// WAIT: [doc(canonica)](https://github.com/rust-lang/rfcs/issues/3011)
#[doc(hidden)]
pub use all::*;
#[doc = crate::_DOC_ALL!()]
pub mod all {
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        _doc::_all::*,
        code::_all::*,
        data::_all::*,
        error::_all::*,
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

#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
// NOTE: module-level `#[rustfmt::skip]` breaks macro import resolution here.
mod _crate_internals {
    //! Crate-internal, hidden items.
    #![allow(unused_imports)]

    #[rustfmt::skip]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        error::_crate_internals::*,
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
        yard::_crate_internals::*,
    };
}

#[doc(hidden)]
pub use _hidden::*;
mod _hidden {
    //! Workspace-public, hidden items.
    #![allow(unused_imports)]

    #[rustfmt::skip]
    #[doc(hidden)]
    pub use super::{
        code::_hidden::*,
        data::_hidden::*,
        // error::_hidden::*,
        geom::_hidden::*,
        media::_hidden::*,
        num::_hidden::*,
        sys::_hidden::*,
        text::_hidden::*,
        work::_hidden::*,
        yard::_hidden::*,
    };
}
