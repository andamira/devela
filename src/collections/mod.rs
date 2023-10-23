// devela::collections
//
//! Collections, extends
//! `std::{`[`array`][mod@std::array],
//! [`collections`][std::collections],
//! [`slice`][std::slice],
//! [`vec`][mod@std::vec]`}`.
//

/* always compiled for internal use */

#[cfg(not(feature = "collections"))]
mod slice;
#[cfg(not(feature = "collections"))]
pub(crate) use slice::*;

/* only compiled with the `collections` feature */

// public modules
#[cfg(feature = "collections")]
pub mod slice;
#[cfg(feature = "collections")]
pub use slice::all::*;

// private modules
#[cfg(feature = "collections")]
mod array;
#[cfg(feature = "collections")]
mod reexports;
#[cfg(feature = "collections")]
mod r#trait;
#[cfg(feature = "collections")]
pub use {array::*, r#trait::*, reexports::*};

#[cfg(feature = "collections")]
pub(crate) mod all {
    // public
    pub use super::slice::all::*;
    // private
    pub use super::{array::*, r#trait::*, reexports::*};
}
