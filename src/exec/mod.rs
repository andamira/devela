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

/* always-compiled */

mod r#async;

pub use r#async::all::*;

/* feature-gated */

#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
mod sync;
#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
mod thread;

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
