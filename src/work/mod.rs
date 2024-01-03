// devela::work
//
//! Work management, extends
//! `std::{`[`future`], [`sync`], [`task`], [`thread`]`}`.
//!
//! [`future`]: core::future
//! [`sync`]: std::sync
//! [`task`]: core::task
//! [`thread`]: std::thread
//

/* contains always compiled items */

// ...

/* feature-gated */

// public sub-modules
#[cfg(feature = "work")]
pub mod r#async;
#[cfg(feature = "work")]
pub mod sync;
#[cfg(feature = "work")]
pub mod thread;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "work")]
#[allow(unused_imports)]
pub use {r#async::all::*, sync::all::*, thread::all::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "work")]
    #[allow(unused_imports)]
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}
