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
mod array;
mod reexports;
mod r#trait;

// feature-gated, non-public
#[cfg(feature = "alloc")]
mod vec_ext;

/* re-exports */

// always compiled, non-public
#[doc(no_inline)]
pub use {array::all::*, r#trait::*, reexports::*};

// feature-gated, non-public
#[cfg(feature = "alloc")]
pub use vec_ext::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{array::all::*, r#trait::*, reexports::*};

    // feature-gated
    #[cfg(feature = "alloc")]
    pub use super::vec_ext::*;
}
