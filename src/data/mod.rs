// devela::data
//
//! Data structures, extends
//! `std::{`[`array`][mod@std::array],
//! [`collections`][std::collections],
//! [`slice`][std::slice],
//! [`vec`][mod@std::vec]`}`.
//

/* contains always compiled items */

pub mod slice;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use slice::*;

/* feature-gated */

#[cfg(feature = "data")]
mod array;
#[cfg(feature = "data")]
mod reexports;
#[cfg(feature = "data")]
mod r#trait;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {array::*, r#trait::*, reexports::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use slice::all::*;

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::*, r#trait::*, reexports::*, slice::all::*};
}
