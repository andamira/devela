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

/* contains always compiled items */

mod array;
pub mod cmp;
pub mod convert;
pub mod error;
pub mod slice;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use {array::*, cmp::*, convert::*, error::*, slice::*};

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
    any(feature = "bytemuck", feature = "dep")
))]
pub use dst::*;
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use {any::all::*, cmp::all::*, convert::all::*, error::*, slice::all::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        any::all::*, array::*, cmp::all::*, collection::DataCollection, convert::all::*, error::*,
        reexports::*, slice::all::*,
    };

    #[doc(inline)]
    #[cfg(all(feature = "unsafe_data", any(feature = "bytemuck", feature = "dep")))]
    pub use super::dst::*;
}
