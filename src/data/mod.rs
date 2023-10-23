// devela::data
//
//! Data structures, extends
//! `std::{`[`array`][mod@std::array],
//! [`collections`][std::collections],
//! [`slice`][std::slice],
//! [`vec`][mod@std::vec]`}`.
//

/* always compiled for internal use */

#[cfg(not(feature = "data"))]
mod slice;
#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use slice::*;

/* only compiled with the `data` feature */

// public modules
#[cfg(feature = "data")]
pub mod slice;
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use slice::all::*;

// private modules
#[cfg(feature = "data")]
mod array;
#[cfg(feature = "data")]
mod reexports;
#[cfg(feature = "data")]
mod r#trait;
#[cfg(feature = "data")]
pub use {array::*, r#trait::*, reexports::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    // public
    pub use super::slice::all::*;
    // private
    pub use super::{array::*, r#trait::*, reexports::*};
}
