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

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_data", feature = "depend")))
)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "depend")
))]
pub mod dst;

#[cfg(feature = "data")]
mod array;
#[cfg(feature = "data")]
mod collection;
#[cfg(feature = "data")]
mod reexports;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {array::*, collection::DataCollection, reexports::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "depend")
))]
pub use dst::*;
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use slice::all::*;

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::*, collection::DataCollection, reexports::*, slice::all::*};

    #[doc(inline)]
    #[cfg(all(feature = "unsafe_data", any(feature = "bytemuck", feature = "depend")))]
    pub use super::dst::*;
}
