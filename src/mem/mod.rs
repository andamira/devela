// devela::mem
//
//! Memory management,
//! <br/><small>extends
//! `std::{`[`alloc`], [`borrow`], [`boxed`], [`cell`], [`mem`], [`pin`],
//! [`ptr`], [`rc`], [`slice`]`}`.</small>
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

mod aligned;
mod ext;
mod fns_macros;
mod reexports;
mod size;
mod slice;
mod storage;
#[allow(unused_imports)]
pub use {aligned::*, ext::*, fns_macros::*, reexports::*, size::*, slice::*, storage::*};

#[cfg(feature = "unsafe_ptr")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ptr")))]
mod pin;
#[cfg(feature = "unsafe_ptr")]
pub use pin::Pinned;

#[cfg(feature = "unsafe_layout")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
mod pod;
#[cfg(feature = "unsafe_layout")]
pub use pod::MemPod;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        aligned::*, ext::*, fns_macros::*, reexports::*, size::all::*, slice::all::*, storage::*,
    };

    #[cfg(feature = "unsafe_ptr")]
    pub use super::pin::Pinned;

    #[cfg(feature = "unsafe_layout")]
    pub use super::pod::MemPod;
}
