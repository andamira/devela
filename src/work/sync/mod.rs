// devela::work::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* modules */

// non-public
mod reexports;

// public
pub mod atomic;

/* re-exports */

// non-public
#[allow(unused_imports)]
pub use reexports::*;

// public
#[doc(no_inline)]
pub use atomic::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{atomic::*, reexports::*};
}
