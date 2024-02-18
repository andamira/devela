// devela::work::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* always compiled, non-public modules */

mod reexports;

#[allow(unused_imports)]
pub use reexports::*;

/* always compiled, non-public modules */

pub mod atomic;

#[doc(no_inline)]
pub use atomic::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{atomic::*, reexports::*};
}
