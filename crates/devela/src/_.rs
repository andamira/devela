//
//! *A development substrate of coherence.*
//!
//! devela is a general-purpose Rust library
//! built from small, reusable abstractions.
//!
//! The modules below organize the library
//! by the kinds of problems they address. <br/>
//! Starting at the crate root, each module
//! highlights a selection from those below.
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
// (In sync with ../Cargo.toml::[workpace.lints.rust.unexpected_cfgs]
//  && ../build/main/features.rs::FLAGS_NIGHTLY)
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))] // enable for all doctests
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))] // configured below
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
//
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
#![cfg_attr(
    all(
        feature = "unsafe_hint",
        any(
            target_arch = "avr",
            target_arch = "msp430",
            target_arch = "xtensa",
            target_arch = "z80",
            all(target_arch = "wasm32", nightly),
        )
    ),
    feature(asm_experimental_arch)
)]
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_float, feature(f16, f128))]
#![cfg_attr(nightly_simd, feature(portable_simd))]
//
// `nightly_stable` includes:
//
// `nightly_stable_1_100`: core, alloc, std:
#![cfg_attr(
    nightly_stable_1_100,
    feature(bool_toggle, drop_guard, mem_conjure_zst, never_type, unsafe_cell_access,)
)]
#![cfg_attr(
    all(nightly_stable_1_100, feature = "alloc"),
    feature(allocator_api, box_take, smart_pointer_try_map,)
)]
#![cfg_attr(all(nightly_stable_1_100, feature = "std"), feature())]
//
// `nightly_stable_1_101`: core, alloc, std:
#![cfg_attr(
    nightly_stable_1_101,
    feature(
        debug_closure_helpers,
        exclusive_wrapper,
        funnel_shifts,
        optimize_attribute,
        ptr_cast_slice,
        slice_split_once,
        unwrap_infallible,
    )
)]
#![cfg_attr(all(nightly_stable_1_101, feature = "alloc"), feature(vec_try_remove, arc_is_unique,))]
// #![cfg_attr(all(nightly_stable_1_101, feature = "std"), feature())]
//
// `nightly_stable_1_102`: core, alloc, std:
// #![cfg_attr(nightly_stable_1_102, feature(,))]
// #![cfg_attr(all(nightly_stable_1_102, feature = "alloc"), feature())]
// #![cfg_attr(all(nightly_stable_1_102, feature = "std"), feature())]
//
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri):
#![cfg_attr(
    nightly_stable_later,
    feature(
        abort_immediate,
        bool_to_result,
        breakpoint,
        cfg_version,
        const_array_from_ref,
        const_char_classify,
        const_slice_from_ref,
        const_sockaddr_setters,
        const_str_split_at,
        // core_io_fundamentals,
        cstr_display,
        derive_coerce_pointee,
        float_bits_const,
        fn_align,
        frontmatter,
        impl_trait_in_assoc_type,
        int_from_ascii,
        isqrt,
        iter_advance_by,
        likely_unlikely,
        macro_metavar_expr,
        macro_metavar_expr_concat,
        more_qualified_paths,
        nonzero_ops,
        offset_of_enum,
        offset_of_slice,
        random, // random_source,
        refcell_try_map,
        str_as_str,
        supertrait_item_shadowing,
        trim_prefix_suffix,
        unsafe_cell_from_mut,
    )
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "alloc"),
    feature(btree_extract_if, new_zeroed_alloc, smart_pointer_map, string_into_chars, vec_from_fn,)
)]
#![cfg_attr(
    all(nightly_stable_later, feature = "std"),
    feature(exitcode_exit_method, once_wait, path_is_empty,) // process_chroot(!miri)
)]
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
        "__dbg", "__std", "__publish", "__select_modules",
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
#[cfg(all(feature = "safe", unsafe··))]
compile_error! {
    concat!("`safe` cannot be combined with `unsafe` or any `unsafe_*` feature.\n",
    "Use individual `safe_*` features to restrict unsafe code in selected modules.")
}

// https://doc.rust-lang.org/nightly/reference/names/preludes.html#extern-prelude
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

extern crate self as devela;

/* root modules */

// `code` bootstraps the module macros; `yard` supplies the shared scaffolding.
// In `__select_modules` mode, other root modules require their own root feature.

#[path = "code/_.rs"] // BOOTSTRAP: provides `mods_in!` and `mods_out!`
pub mod code; // Code structure, semantics, and foundational utilities

crate::mods_in! {
    pub mod_ _doc; // Extra documentation about the library

    #[macro_apply(__doc_auto_hide_features(("data")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    pub mod_ data; // Structural abstractions for organizing and manipulating data

    #[macro_apply(__doc_auto_hide_features(("error")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "error"))]
    pub mod_ error; // Failure types, result aliases, and recovery semantics

    #[macro_apply(__doc_auto_hide_features(("geom")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    pub mod_ geom; // Geometric types, operations, and spatial constructs

    #[macro_apply(__doc_auto_hide_features(("lang")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "lang"))]
    pub mod_ lang; // Language structure and meaning across domains

    #[macro_apply(__doc_auto_hide_features(("media")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    pub mod_ media; // Media formats, representation, and synthesis

    #[macro_apply(__doc_auto_hide_features(("num")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    pub mod_ num; // Numeric abstractions and formal computation

    #[macro_apply(__doc_auto_hide_features(("org")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "org"))]
    pub mod_ org; // Coordination and structure of collective action

    #[macro_apply(__doc_auto_hide_features(("phys")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "phys"))]
    pub mod_ phys; // Physical quantities, units, and models of the natural world

    #[macro_apply(__doc_auto_hide_features(("run")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "run"))]
    pub mod_ run; // Temporal coordination and staging of a running system

    #[macro_apply(__doc_auto_hide_features(("ui")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    pub mod_ ui; // Interactive surfaces, input semantics, and presentation state

    #[macro_apply(__doc_auto_hide_features(("vita")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "vita"))]
    pub mod_ vita; // Lived practices of embodied beings

    #[macro_apply(__doc_auto_hide_features(("sys")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub mod_ sys; // System interfaces and hardware abstractions

    #[macro_apply(__doc_auto_hide_features(("text")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    pub mod_ text; // Symbolic sequences, encodings, and text processing

    #[macro_apply(__doc_auto_hide_features(("work")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    pub mod_ work; // Computational work, its coordination, and its execution

    // internal
    pub mod_ yard; // Scaffolding, taxonomy, and documentation support.
}
#[doc(hidden)]
pub use yard::dep as _dep;

/* manual mods_out!: */

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
    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("data")))))]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }

    #[doc = concat![crate::_DOC_ERROR!(), crate::_DOC_ERROR_MODULES!(), _COMMON_DOC!("error")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "error"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("error")))))]
    pub mod _error { #[allow(unused)] pub use super::super::error::_all::*; }

    #[doc = concat![crate::_DOC_GEOM!(), crate::_DOC_GEOM_MODULES!(), _COMMON_DOC!("geom")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("geom")))))]
    pub mod _geom { #[allow(unused)] pub use super::super::geom::_all::*; }

    #[doc = concat![crate::_DOC_LANG!(), crate::_DOC_LANG_MODULES!(), _COMMON_DOC!("lang")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "lang"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("lang")))))]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }

    #[doc = concat![crate::_DOC_MEDIA!(), crate::_DOC_MEDIA_MODULES!(), _COMMON_DOC!("media")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("media")))))]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }

    #[doc = concat![crate::_DOC_NUM!(), crate::_DOC_NUM_MODULES!(), _COMMON_DOC!("num")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("num")))))]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }

    #[doc = crate::_tags!(wip)]
    #[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
    #[doc = concat![crate::_DOC_ORG!(), crate::_DOC_ORG_MODULES!(), _COMMON_DOC!("org")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "org"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("org")))))]
    pub mod _org { #[allow(unused)] pub use super::super::org::_all::*; }

    #[doc = concat![crate::_DOC_PHYS!(), crate::_DOC_PHYS_MODULES!(), _COMMON_DOC!("phys")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "phys"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("phys")))))]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }

    #[doc = concat![crate::_DOC_RUN!(), crate::_DOC_RUN_MODULES!(), _COMMON_DOC!("run")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "run"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("run")))))]
    pub mod _run { #[allow(unused)] pub use super::super::run::_all::*; }

    #[doc = concat![crate::_DOC_SYS!(), crate::_DOC_SYS_MODULES!(), _COMMON_DOC!("sys")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("sys")))))]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }

    #[doc = concat![crate::_DOC_TEXT!(), crate::_DOC_TEXT_MODULES!(), _COMMON_DOC!("text")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("text")))))]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }

    #[doc = concat![crate::_DOC_UI!(), crate::_DOC_UI_MODULES!(), _COMMON_DOC!("ui")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("ui")))))]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }

    #[doc = crate::_tags!(wip)]
    #[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
    #[doc = concat![crate::_DOC_VITA!(), crate::_DOC_VITA_MODULES!(), _COMMON_DOC!("vita")]]
    #[cfg(any(not(feature = "__select_modules"), feature = "vita"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("vita")))))]
    pub mod _vita { #[allow(unused)] pub use super::super::vita::_all::*; }

    #[doc = concat![crate::_DOC_WORK!(), crate::_DOC_WORK_MODULES!(), _COMMON_DOC!("work")]]
    /// <br/><hr>
    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("work")))))]
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

// public items, feature-gated, visible at their origin and in `all`:
// WAIT: [doc(canonica)](https://github.com/rust-lang/rfcs/issues/3011)
#[doc(hidden)]
pub use all::*;
#[doc = crate::_DOC_ALL!()]
pub mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::code::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("data")))))]
    pub use super::data::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "error"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("error")))))]
    pub use super::error::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("geom")))))]
    pub use super::geom::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "lang"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("lang")))))]
    pub use super::lang::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("media")))))]
    pub use super::media::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("num")))))]
    pub use super::num::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "org"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("org")))))]
    pub use super::org::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "phys"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("phys")))))]
    pub use super::phys::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "run"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("run")))))]
    pub use super::run::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("sys")))))]
    pub use super::sys::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("text")))))]
    pub use super::text::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("ui")))))]
    pub use super::ui::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "vita"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("vita")))))]
    pub use super::vita::_all::*;

    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("work")))))]
    pub use super::work::_all::*;
}

#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
// NOTE: module-level `#[rustfmt::skip]` breaks macro import resolution here.
mod _crate_internals {
    //! Crate-internal, hidden items.
    #![allow(unused_imports)]

    pub(crate) use super::code::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    pub(crate) use super::data::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "error"))]
    pub(crate) use super::error::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    pub(crate) use super::geom::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "lang"))]
    pub(crate) use super::lang::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    pub(crate) use super::media::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    pub(crate) use super::num::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "org"))]
    pub(crate) use super::org::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "phys"))]
    pub(crate) use super::phys::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "run"))]
    pub(crate) use super::run::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub(crate) use super::sys::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    pub(crate) use super::text::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    pub(crate) use super::ui::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "vita"))]
    pub(crate) use super::vita::_crate_internals::*;

    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    pub(crate) use super::work::_crate_internals::*;

    pub(crate) use super::yard::_crate_internals::*;
}

#[doc(hidden)]
pub use __hidden::*;
#[doc(hidden)]
pub mod __hidden {
    //! Workspace-public, hidden items.
    #![allow(unused_imports)]

    #[doc(hidden)]
    pub use super::code::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    pub use super::data::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    pub use super::geom::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    pub use super::media::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    pub use super::num::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub use super::sys::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    pub use super::text::_hidden::*;

    #[doc(hidden)]
    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    pub use super::work::_hidden::*;

    #[doc(hidden)]
    pub use super::yard::_hidden::*;
}

// Highlights
#[doc(inline)]
#[allow(unused_imports)]
pub use _reexports::*;
#[rustfmt::skip]
mod _reexports { // 39
    #![allow(unused_imports)]

    // code: 3
    #[doc(inline)]
    pub use super::code::{ConstInit, const_assert, unwrap};

    // data: 5
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "data"))]
    pub use super::data::{ByteCursor, WordTry, arena, handle, pool};

    // error: 0
    // pub use super::error::{};

    // geom: 4
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "geom"))]
    pub use super::geom::{Angle, Extent, Point, Region};

    // lang: 1
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "lang"))]
    pub use super::lang::ScriptMachine;

    // media: 2
    #[doc(inline)] #[cfg(feature = "draw")]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    pub use super::media::Canvas;
    #[doc(inline)] #[cfg(feature = "audio")]
    #[cfg(any(not(feature = "__select_modules"), feature = "media"))]
    pub use super::media::PcmBuf;

    // num: 5
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "num"))]
    pub use super::num::{Cast, Cmp, Interval, RandTry, Ratio};

    // org: 0
    // pub use super::org::{};

    // phys: 2
    #[doc(inline)] #[cfg(feature = "time")]
    #[cfg(any(not(feature = "__select_modules"), feature = "phys"))]
    pub use super::phys::{TimeDelta, TimeSource};

    // run: 2
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "run"))]
    pub use super::run::{RunApp, RunDriver};

    // sys: 5
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub use super::sys::{Slice, TextOut};
    #[doc(inline)] #[cfg(feature = "_linux_abi")]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub use super::sys::Linux;
    #[doc(inline)] #[cfg(feature = "term")]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub use super::sys::TermGrid;
    #[doc(inline)] #[cfg(all(feature = "web", not(windows), not(feature = "safe_sys")))]
    #[cfg(any(not(feature = "__select_modules"), feature = "sys"))]
    pub use super::sys::Web;

    // text: 6
    #[doc(inline)]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    pub use super::text::{CharIter, Str, StringU8, TextScanner, format_buf};
    #[doc(inline)] #[cfg(feature = "grapheme")]
    #[cfg(any(not(feature = "__select_modules"), feature = "text"))]
    pub use super::text::GraphemeMachine;

    // ui: 2
    #[doc(inline)] #[cfg(feature = "event")]
    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    pub use super::ui::Event;
    #[doc(inline)] #[cfg(feature = "ui")]
    #[cfg(any(not(feature = "__select_modules"), feature = "ui"))]
    pub use super::ui::TextInput;

    // vita: 0
    // pub use super::vita::{};

    // work: 2
    #[doc(inline)] #[cfg(feature = "std")]
    #[cfg(any(not(feature = "__select_modules"), feature = "work"))]
    pub use super::work::{CommandFlow, cmd};
}
