// devela::collections
//
//! Collections, extends [`alloc::collections`]`/`[`vec`][mod@alloc::vec].
//

/* always compiled for internal use */

/* only compiled with the `collections` feature */

/* re-exports */

#[cfg(feature = "collections")]
mod reexports;

#[cfg(feature = "collections")]
pub use all::*;
#[cfg(feature = "collections")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
