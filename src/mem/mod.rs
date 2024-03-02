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

/* always compiled, non-public modules */

mod aligned;
mod fns_macros;
mod reexports;
mod size;
mod slice;
mod storage;
mod r#trait;

#[allow(unused_imports)]
pub use {aligned::*, fns_macros::*, r#trait::*, reexports::*, size::*, slice::*, storage::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        aligned::*, fns_macros::*, r#trait::*, reexports::*, size::all::*, slice::all::*,
        storage::*,
    };
}
