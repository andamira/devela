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
mod sync;
mod thread;

pub use {r#async::all::*, sync::all::*, thread::all::*};

pub(crate) mod all {
    // always-compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}