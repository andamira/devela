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

//

/* contains always compiled items */

mod aligned;
mod always;
mod reexports;
mod storage;
mod r#trait;

pub mod size;
pub mod slice;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub use {aligned::*, always::*, r#trait::*, reexports::*, size::*, slice::*, storage::*};

/* feature-gated */

#[cfg(feature = "mem")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {aligned::*, always::*, fns::*, r#trait::*, reexports::*, storage::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "mem")]
pub use {size::all::*, slice::all::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        aligned::*, always::*, r#trait::*, reexports::*, size::all::*, slice::all::*, storage::*,
    };

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::fns::*;
}
