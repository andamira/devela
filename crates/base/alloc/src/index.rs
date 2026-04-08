// devela_base_alloc::index
//
//! A development substrate of coherence.
//!
//! Base library built on Rust's `alloc`.
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
//
// documentation
//
// #![cfg_attr(nightly_doc, doc(auto_cfg = true))]

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

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_core::zall::*;
    pub use devela_base_core::_workspace_internals::*;
}
