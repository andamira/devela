// devela::collections
//
//! Collections, extends [`std::collections`] /
//! [`array`][mod@std::array] /
//! [`vec`][mod@std::vec].
//

/* always compiled for internal use */

/* only compiled with the `collections` feature */

#[cfg(feature = "collections")]
mod array;
#[cfg(feature = "collections")]
mod r#trait;

/* re-exports */

#[cfg(feature = "collections")]
mod reexports;

#[cfg(feature = "collections")]
pub use all::*;
#[cfg(feature = "collections")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::*, r#trait::*, reexports::*};
}
