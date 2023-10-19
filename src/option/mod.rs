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

    crate::codegen::reexport! { "const-str" | const_str , features: "option", "str",
        doc: "Returns an unwrapped [`Option<T: Copy>`] in compile-time.",
        @unwrap as option_unwrap
    }
}
