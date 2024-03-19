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

/* always compiled */

mod aligned;
mod ext;
mod fns_macros;
mod reexports;
mod size;
mod slice;
mod storage;

#[allow(unused_imports)]
pub use {aligned::*, ext::*, fns_macros::*, reexports::*, size::*, slice::*, storage::*};

/* feature-gated */

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        aligned::*, ext::*, fns_macros::*, reexports::*, size::all::*, slice::all::*, storage::*,
    };
}
