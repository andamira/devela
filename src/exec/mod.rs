// devela::exec
//
//! Execution strategies and concurrency management, <small>extends
//! `std::{`[`future`], [`process`], [`sync`], [`task`], [`thread`]`}`.</small>
//!
//! [`future`]: std::future
//! [`process`]: std::process
//! [`sync`]: std::sync
//! [`task`]: std::task
//! [`thread`]: std::thread
//

// safety:
#![cfg_attr(feature = "safe_exec", forbid(unsafe_code))]

/* always-compiled, public modules */

pub mod r#async;

#[doc(no_inline)]
pub use r#async::all::*;

/* feature-gated, public modules */

#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub mod sync;
#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub mod thread;

#[doc(no_inline)]
#[cfg(feature = "exec")]
#[allow(unused_imports)]
pub use {sync::all::*, thread::all::*};

pub(crate) mod all {
    // always-compiled
    #[doc(inline)]
    pub use super::r#async::all::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "exec")]
    #[allow(unused_imports)]
    pub use super::{sync::all::*, thread::all::*};
}
