// devela_base_alloc::index
//
//!
//

/* crate configuration */
//
// lints
// #![allow(unexpected_cfgs)]
//
// environment
#![cfg_attr(not(feature = "__std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
// nightly
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
#![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
// #![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
// #![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
// #![cfg_attr(nightly_float, feature(f16, f128))]
// #![cfg_attr(nightly_simd, feature(portable_simd))]

extern crate alloc;
// NOTE: for testing purposes only
#[cfg(feature = "__std")]
extern crate std;

extern crate self as devela_base_alloc;
macro_rules! __crate_name {
    () => {
        "devela_base_alloc"
    };
}
#[allow(unused_imports)]
pub(crate) use __crate_name;

#[cfg(feature = "alloc")]
items! {
    pub mod code;
    pub mod data;
    pub mod geom;
    pub mod lang;
    pub mod media;
    pub mod num;
    pub mod phys;
    pub mod run;
    pub mod sys;
    pub mod text;
    pub mod ui;
    pub mod work;
}
// internal:
pub mod yard;
#[doc(hidden)]
pub use yard::_dep;
//
// mod _doc;

#[doc(hidden)]
#[allow(unused_imports)]
pub use zall::*;
pub mod zall {
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::{
        code::_all::*,
        data::_all::*,
        geom::_all::*,
        lang::_all::*,
        media::_all::*,
        num::_all::*,
        phys::_all::*,
        run::_all::*,
        sys::_all::*,
        text::_all::*,
        ui::_all::*,
        work::_all::*,
    };
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
mod _crate_internals {
    #[rustfmt::skip]
    #[cfg(feature = "alloc")]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        geom::_crate_internals::*,
        lang::_crate_internals::*,
        media::_crate_internals::*,
        num::_crate_internals::*,
        phys::_crate_internals::*,
        run::_crate_internals::*,
        sys::_crate_internals::*,
        text::_crate_internals::*,
        ui::_crate_internals::*,
        work::_crate_internals::*,
    };
}

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_core::zall::*;
    pub use devela_base_core::_workspace_internals::*;
}
