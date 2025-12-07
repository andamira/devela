// devela_base_num::index
//
//!
//

/* crate configuration */
// environment
#![cfg_attr(not(feature = "__std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly
// nightly (uncomment as used)
// #![cfg_attr(nightly_allocator, feature(allocator_api))]
// #![cfg_attr(nightly_autodiff, feature(autodiff))] // FLAG_DISABLED:nightly_autodiff
// #![cfg_attr(nightly_become, feature(explicit_tail_calls))] // WARN:incomplete_features
// #![cfg_attr(nightly_bigint, feature(bigint_helper_methods))]
// #![cfg_attr(nightly_coro, feature(coroutines, coroutine_trait, iter_from_coroutine))]
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
// #![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]
// #![cfg_attr(nightly_float, feature(f16, f128))]
// #![cfg_attr(nightly_simd, feature(portable_simd))]

extern crate self as devela_base_num;

pub mod num;

#[doc(hidden)]
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
    pub use super::{
        num::_all::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use _crate_internals::*;
#[doc(hidden)]
mod _crate_internals {
    #[doc(hidden)]
    #[rustfmt::skip]
    pub use {
        devela_base_core::_workspace_internals::*,
        devela_base_core::zall::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use _workspace_internals::*;
#[doc(hidden)]
pub mod _workspace_internals {
    #[doc(hidden)]
    #[rustfmt::skip]
    #[allow(unused_imports)]
    pub use super::{
        num::_workspace_internals::*,
    };
}
