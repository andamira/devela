// devela::result::option
//
//! Option, extends
//! `std::`[`option`][std::option].
//

mod ext;
mod fmt;

mod reexports {
    crate::code::reexport! { "const-str" | const_str , features: "result", "text",
        doc: "Returns an unwrapped [`Option<T: Copy>`] in compile-time.",
        @unwrap as option_unwrap
    }
}

// re-export private sub-modules
pub use {
    ext::OptionExt,
    fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    reexports::*,
};

pub(crate) mod all {
    pub use super::{ext::*, fmt::*, reexports::*};
}
