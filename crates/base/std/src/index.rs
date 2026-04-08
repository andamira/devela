// devela_base_std::index
//
//! A development substrate of coherence.
//!
//! Base library built on Rust's `std`.
//

/* crate configuration */
//
// lints
// #![allow(unexpected_cfgs)]
//
// environment
#![cfg_attr(not(feature = "std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly (uncomment as used)
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

extern crate self as devela_base_std;
macro_rules! __crate_name {
    () => {
        "devela_base_std"
    };
}
#[allow(unused_imports)]
pub(crate) use __crate_name;

#[cfg(feature = "std")]
items! {
    pub mod build;
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
