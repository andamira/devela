// devela::option
//
//! Option, extends [`core::option`].
//!
//! It also reexports the [`const-str`](https://docs.rs/const-str) `unwrap` macro.
//

mod ext;
mod fmt;

pub use ext::OptionExt;
pub use fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse};

#[cfg(feature = "const-str")]
#[doc = "Returns an unwrapped [`Option<T: Copy>`] in compile-time.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
pub use const_str::unwrap as option_unwrap;
