// devela::data::collections
//
//! Data collections, extends
//! `std::{`[`array`], [`collections`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`vec`]: std::vec
//

#![allow(unused_imports)]

/* modules */

// always compiled, non-public
mod adt;
mod array;
mod reexports;

// feature-gated, non-public
#[cfg(feature = "alloc")]
mod ext_vec;

/* re-exports */

// always compiled, non-public
pub use {adt::*, array::all::*, reexports::*};

// feature-gated, non-public
#[cfg(feature = "alloc")]
pub use ext_vec::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{adt::*, array::all::*, reexports::*};

    // feature-gated
    #[cfg(feature = "alloc")]
    pub use super::ext_vec::*;
}
