// devela_base_core::index
//
//!
//

/* crate configuration */
//
// lints
// #![allow(unexpected_cfgs)]
#![cfg_attr(
    not(all(doc, feature = "_docs")), // if features are incomplete…
    allow(rustdoc::broken_intra_doc_links) // …allow broken intra-doc links
)]
// environment
#![cfg_attr(not(feature = "__std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
//
// #![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_float, feature(f16, f128))]
#![cfg_attr(nightly_simd, feature(portable_simd))]

/* crate safeguards */

// safety
#[cfg(all(
    feature = "safe",
    // In sync with ../Cargo.toml::unsafe & /build/main/features.rs::UNSAFE
    any(feature = "unsafe", // includes all 11 specific purposes below:
        feature = "unsafe_array", feature = "unsafe_ffi", feature = "unsafe_hint",
        feature = "unsafe_layout", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str", feature = "unsafe_sync",
        feature = "unsafe_syscall", feature = "unsafe_thread",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// (note: you can enable `safe_*` features to prevent `unsafe` use in specific modules)

extern crate self as devela_base_core;
macro_rules! __crate_name {
    () => {
        "devela_base_core"
    };
}
pub(crate) use __crate_name;

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
//
mod yard;
#[doc(hidden)]
pub use yard::_dep;

mod _doc;

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
    #[doc = concat![crate::_DOC_ORG!(), crate::_DOC_ORG_MODULES!(), COMMON_DOC!("org")]]
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
    #[doc = concat![crate::_DOC_VITA!(), crate::_DOC_VITA_MODULES!(), COMMON_DOC!("vita")]]
    pub mod _vita { #[allow(unused)] pub use super::super::vita::_all::*; }
    #[doc = concat![crate::_DOC_WORK!(), crate::_DOC_WORK_MODULES!(), COMMON_DOC!("work")]]
    /// <br/><hr>
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

// public items, feature-gated, visible at their origin and in `zall`:
// WAIT: [doc(canonica)](https://github.com/rust-lang/rfcs/issues/3011)
#[doc(hidden)]
pub use {zall as all, zall::*}; // keep devela_base_core::all::* accesor hidden
#[doc = crate::_DOC_ZALL!()]
pub mod zall {
    #[doc(inline)]
    #[rustfmt::skip]
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

#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _crate_internals {
    #![allow(unused_imports)]
    pub use super::{
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
        yard::_crate_internals::*,
    };
}
#[doc(hidden)]
#[allow(unused_imports)]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use super::{
        _doc::_workspace_internals::*,
        code::_workspace_internals::*,
        num::_workspace_internals::*,
        yard::_workspace_internals::*,
    };
}

// public, hidden items
#[doc(hidden)]
#[allow(unused_imports)]
pub use _hidden::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _hidden {
    #[allow(unused_imports)]
    pub use super::{
        num::_hidden::*,
        sys::_hidden::*,
    };
}
