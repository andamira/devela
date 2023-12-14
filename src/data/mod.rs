// devela::data
//
//! Data structures, extends
//! `std::{`[`any`], [`array`], [`cmp`], [`collections`], [`convert`], [`vec`]`}`.
//!
//! [`any`]: core::any
//! [`array`]: mod@core::array
//! [`cmp`]: core::cmp
//! [`collections`]: alloc::collections
//! [`convert`]: core::convert
//! [`vec`]: mod@alloc::vec
//

#![cfg_attr(not(feature = "data"), allow(unused))]

/* contains always compiled items */

mod collection;
mod error;
mod reexports;

pub mod array;
pub mod bit;
pub mod cmp;
pub mod convert;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use {array::*, bit::*, cmp::*, collection::*, convert::*, error::*, reexports::*};

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
pub use {any::all::*, array::all::*, bit::all::*, cmp::all::*, convert::all::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        array::all::*, bit::all::*, cmp::all::*, collection::*, convert::all::*, error::*,
        reexports::*,
    };

    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::any::all::*;

    #[doc(inline)]
    #[cfg(all(feature = "unsafe_data", any(feature = "bytemuck", feature = "dep")))]
    pub use super::dst::*;
}
