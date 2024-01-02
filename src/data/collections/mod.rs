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

/* contains always compiled items */

mod array;
mod reexports;
mod r#trait;

#[cfg(not(feature = "data"))]
pub use {array::*, r#trait::*, reexports::*};

/* feature-gated */

// private sub-modules

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {array::all::*, r#trait::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::all::*, r#trait::*, reexports::*};
}
