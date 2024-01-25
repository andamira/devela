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

/* modules */

// feature-gated, public
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "work")))]
pub mod r#async;
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "work")))]
pub mod sync;
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "work")))]
pub mod thread;

/* re-exports */

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "work")]
#[allow(unused_imports)]
pub use {r#async::all::*, sync::all::*, thread::all::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "work")]
    #[allow(unused_imports)]
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}
