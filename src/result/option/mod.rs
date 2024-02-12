// devela::result::option
//
//! Option, extends
//! `std::`[`option`][std::option].
//

mod ext;
mod fmt;

// re-export private sub-modules
#[allow(unused_imports)]
pub use {
    ext::ExtOption,
    fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext::*, fmt::*};
}
