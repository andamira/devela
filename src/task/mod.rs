// devela::task
//
//! Tasking, extends
//! `std::{`[`future`][std::future],
//! [`sync`][std::sync],
//! [`task`][std::task],
//! [`thread`][std::thread]`}`.
//

/* contains always compiled items */

// ...

/* feature-gated */

// public sub-modules
#[cfg(feature = "task")]
pub mod r#async;
#[cfg(feature = "task")]
pub mod sync;
#[cfg(feature = "task")]
pub mod thread;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "task")]
pub use {r#async::all::*, sync::all::*, thread::all::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "task")]
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}
