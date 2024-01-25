// devela::mem
//
//! Memory management, extends
//! `std::{`[`alloc`], [`borrow`], [`boxed`], [`cell`], [`mem`], [`pin`],
//! [`ptr`], [`rc`], [`slice`]`}`.
//!
//! [`alloc`]: core::alloc
//! [`borrow`]: core::borrow
//! [`boxed`]: std::boxed
//! [`cell`]: core::cell
//! [`mem`]: core::mem
//! [`pin`]: core::pin
//! [`ptr`]: core::ptr
//! [`rc`]: std::rc
//! [`slice`]: core::slice

/* modules */

// always compiled, non-public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod aligned;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod always;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod reexports;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod slice;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod storage;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
mod r#trait;

// always compiled, public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub mod size;

// feature-gated, non-public
#[cfg(feature = "mem")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
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
