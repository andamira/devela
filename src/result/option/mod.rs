// devela::result::option
//
//! Option, extends `std::`[`option`][std::option].
//

/* always compiled for internal use */

/* only compiled with the `option` feature */

#[cfg(feature = "result")]
mod ext;
#[cfg(feature = "result")]
mod fmt;

/* re-exports */

#[cfg(feature = "result")]
pub use all::*;
#[cfg(feature = "result")]
pub(crate) mod all {
    pub use super::{
        ext::OptionExt,
        fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    };

    crate::codegen::reexport! { "const-str" | const_str , features: "result", "text",
        doc: "Returns an unwrapped [`Option<T: Copy>`] in compile-time.",
        @unwrap as option_unwrap
    }
}
