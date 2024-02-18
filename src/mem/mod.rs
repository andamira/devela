// devela::mem
//
//! Memory management, extends
//! `std::{`[`alloc`], [`borrow`], [`boxed`], [`cell`], [`mem`], [`pin`],
//! [`ptr`], [`rc`], [`slice`]`}`.
//!
//! [`alloc`]: std::alloc
//! [`borrow`]: std::borrow
//! [`boxed`]: std::boxed
//! [`cell`]: std::cell
//! [`mem`]: std::mem
//! [`pin`]: std::pin
//! [`ptr`]: std::ptr
//! [`rc`]: std::rc
//! [`slice`]: std::slice

// safety:
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

/* modules */

// always compiled, non-public
mod aligned;
mod always;
mod reexports;
mod size;
mod slice;
mod storage;
mod r#trait;

// feature-gated, non-public
#[cfg(feature = "mem")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mem")))]
mod fns;

/* re-exports */

// always compiled, non-public
#[allow(unused_imports)]
pub use {aligned::*, always::*, r#trait::*, reexports::*, size::*, slice::*, storage::*};

// feature-gated, non-public
#[cfg(feature = "mem")]
pub use fns::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        aligned::*, always::*, r#trait::*, reexports::*, size::all::*, slice::all::*, storage::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::fns::*;
}
