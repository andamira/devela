// devela_base_core::lib
//
//!
//

/* crate configuration */
// environment
#![cfg_attr(not(feature = "__std"), no_std)]
// safety
#![cfg_attr(base_safe, forbid(unsafe_code))]
//
// nightly (uncomment as used)
// #![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
// #![cfg_attr(nightly_bigint, feature(bigint_helper_methods))]
#![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
#![cfg_attr(nightly_float, feature(f16, f128))]
// #![cfg_attr(nightly_simd, feature(portable_simd))]

/* crate safeguards */

// safety
#[cfg(all(
    base_safe, // alias: all(feature = "base_safe", feature = "safe")
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

#[rustfmt::skip]
pub mod _doc {
    //! Extra documentation about the library.
    #[doc = crate::_DOC_CODE!()]
    pub mod _code { #[allow(unused)] pub use super::super::code::_all::*; }
    #[doc = crate::_DOC_DATA!()]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }
    #[doc = crate::_DOC_GAME!()]
    pub mod _game { #[allow(unused)] pub use super::super::game::_all::*; }
    #[doc = crate::_DOC_LANG!()]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }
    #[doc = crate::_DOC_MEDIA!()]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }
    #[doc = crate::_DOC_NUM!()]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }
    #[doc = crate::_DOC_PHYS!()]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }
    #[doc = crate::_DOC_SYS!()]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }
    #[doc = crate::_DOC_TEXT!()]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }
    #[doc = crate::_DOC_UI!()]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }
    #[doc = crate::_DOC_WORK!()]
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

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
#[allow(unused_imports)]
pub use _hidden::*;
mod _hidden {
    #[allow(unused_imports)]
    pub use super::sys::_hidden::*;
}

#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _crate_internals {
    #![allow(unused_imports)]
    pub use super::{
        code::_crate_internals::*,
        ui::_crate_internals::*,
    };
}
#[doc(hidden)]
#[allow(unused_imports)]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use super::{
        code::_workspace_internals::*,
        data::_workspace_internals::*,
        num::_workspace_internals::*,
    };
}
