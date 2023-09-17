// devela::str
//
//! String slices, extends [`core::str`].
//!
//! It also reexports most [`const-str`](https://docs.rs/const-str) macros
//! directly related to [`&str`], prefixed with `str_`, and a new description.
//

mod ext;
pub use ext::StrExt;

mod reexport_const_str;
pub use reexport_const_str::*;
