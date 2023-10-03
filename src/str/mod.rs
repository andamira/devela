// devela::str
//
//! String slices, extends [`core::str`].
//!
//! It also reexports most [`const-str`](https://docs.rs/const-str) macros
//! directly related to [`&str`], prefixed with `str_`, and a new description.
//

/* always compiled for internal use */

/* only compiled with the `ops` feature */

#[cfg(feature = "str")]
mod ext;

#[cfg(feature = "str")]
mod reexport_const_str;

/* re-exports */

#[cfg(feature = "str")]
pub use all::*;
#[cfg(feature = "str")]
pub(crate) mod all {
    pub use super::{ext::StrExt, reexport_const_str::*};
}
