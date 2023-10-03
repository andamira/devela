// devela::option
//
//! Option, extends [`core::option`].
//!
//! It also reexports the [`const-str`](https://docs.rs/const-str) `unwrap` macro.
//

/* always compiled for internal use */

/* only compiled with the `option` feature */

#[cfg(feature = "option")]
mod ext;
#[cfg(feature = "option")]
mod fmt;

/* re-exports */

#[cfg(feature = "option")]
pub use all::*;
#[cfg(feature = "option")]
pub(crate) mod all {
    pub use super::{
        ext::OptionExt,
        fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    };

    #[cfg(feature = "const-str")]
    #[doc = "Returns an unwrapped [`Option<T: Copy>`] in compile-time.\n\n"]
    #[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
    pub use const_str::unwrap as option_unwrap;
}
