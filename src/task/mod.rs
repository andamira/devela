// devela::task
//
//! Tasking, extends
//! `std::{`[`future`][std::future],
//! [`sync`][std::sync],
//! [`task`][std::task],
//! [`thread`][std::thread]`}`.
//

/* always compiled for internal use */

/* only compiled with the `task` feature */

#[cfg(feature = "task")]
pub mod r#async;
#[cfg(feature = "task")]
pub mod sync;
#[cfg(feature = "task")]
pub mod thread;
#[doc(no_inline)]
#[cfg(feature = "task")]
pub use {r#async::all::*, sync::all::*, thread::all::*};

/* re-exports */

#[cfg(feature = "task")]
pub(crate) mod all {
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}
