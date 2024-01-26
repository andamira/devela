// devela::error::option
//
//! Option, extends
//! `std::`[`option`][std::option].
//

mod ext;
mod fmt;

mod reexports {
    crate::code::reexport! { "const-str" | const_str , features: "error", "text",
        doc: "Returns an unwrapped [`Option<T: Copy>`] in compile-time.",
        @unwrap as option_unwrap
    }
}

// re-export private sub-modules
#[allow(unused_imports)]
pub use {
    ext::ExtOption,
    fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    reexports::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext::*, fmt::*, reexports::*};
}
