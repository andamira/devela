// devela::data
//
//! Data structures, extends
//! `std::{`[`any`][std::any],
//! [`array`][mod@std::array],
//! [`cmp`][std::cmp],
//! [`collections`][std::collections],
//! [`convert`][std::convert],
//! [`slice`][std::slice],
//! [`vec`][mod@std::vec]`}`.
//

#![cfg_attr(not(feature = "data"), allow(unused))]

/* contains always compiled items */

mod collection;
mod error;

pub mod array;
pub mod bit;
pub mod cmp;
pub mod convert;
mod reexports;
pub mod slice;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use {
    array::*, bit::*, cmp::*, collection::*, convert::*, error::*, reexports::*, slice::*,
};

/* feature-gated */

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_data", feature = "dep")))
)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "dep")
))]
pub mod dst;

#[cfg(feature = "data")]
pub mod any;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {collection::*, error::*, reexports::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "dep")
))]
pub use dst::*;
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use {any::all::*, array::all::*, bit::all::*, cmp::all::*, convert::all::*, slice::all::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        any::all::*, array::all::*, bit::all::*, cmp::all::*, collection::*, convert::all::*,
        error::*, reexports::*, slice::all::*,
    };

    #[doc(inline)]
    #[cfg(all(feature = "unsafe_data", any(feature = "bytemuck", feature = "dep")))]
    pub use super::dst::*;
}
