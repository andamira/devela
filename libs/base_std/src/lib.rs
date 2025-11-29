// devela_base_std::lib
//
//!
//

/* crate configuration */
// environment
#![cfg_attr(not(feature = "std"), no_std)]
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

extern crate self as devela_base_std;

#[cfg(feature = "std")]
items! {
    pub mod build;
    pub mod code;
    pub mod data;
    pub mod media;
    pub mod num;
    pub mod phys;
    pub mod sys;
    pub mod text;
    pub mod work;
}

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
    #[cfg(feature = "std")]
    pub use super::{
        build::_all::*,
        code::_all::*,
        data::_all::*,
        media::_all::*,
        num::_all::*,
        phys::_all::*,
        sys::_all::*,
        text::_all::*,
        work::_all::*,
    };
}

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_alloc::zall::*;
    pub use devela_base_alloc::_workspace_internals::*;
}
