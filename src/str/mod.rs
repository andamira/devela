// devela::str
//
//! String slices, extends [`core::str`].
//!
//! It also reexports most [`const-str`](https://docs.rs/const-str) macros
//! directly related to [`&str`], prefixed with `str_`.
//

/* always compiled for internal use */

/* only compiled with the `ops` feature */

#[cfg(feature = "str")]
mod ext;

/* re-exports */

#[cfg(feature = "str")]
mod reexport;

#[cfg(feature = "str")]
pub use all::*;
#[cfg(feature = "str")]
pub(crate) mod all {
    pub use super::ext::StrExt;

    pub use super::reexport::*;
}
