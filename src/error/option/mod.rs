// devela::error::option
//
//! Option, extends
//! `std::`[`option`][std::option].
//

mod ext;
mod fmt;
mod optres;

// re-export private sub-modules
#[allow(unused_imports)]
pub use {
    ext::ExtOption,
    fmt::{OptionFmt, OptionFmtOr, OptionFmtOrElse},
    optres::{serr, sok, OptRes},
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext::*, fmt::*, optres::*};
}
