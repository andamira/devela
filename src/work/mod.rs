// devela::work
//
//! Work management, extends
//! `std::{`[`future`], [`sync`], [`task`], [`thread`]`}`.
//!
//! [`future`]: std::future
//! [`sync`]: std::sync
//! [`task`]: std::task
//! [`thread`]: std::thread
//

// safety:
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

/* always-compiled, public modules */

pub mod r#async;

#[doc(no_inline)]
pub use r#async::all::*;

/* feature-gated, public modules */

#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub mod sync;
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub mod thread;

#[doc(no_inline)]
#[cfg(feature = "work")]
#[allow(unused_imports)]
pub use {sync::all::*, thread::all::*};

pub(crate) mod all {
    // always-compiled
    #[doc(inline)]
    pub use super::r#async::all::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "work")]
    #[allow(unused_imports)]
    pub use super::{sync::all::*, thread::all::*};
}
