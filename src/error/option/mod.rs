// devela::error::option
//
//! Optional values.
#![doc = crate::doc_!(extends: option)]
#![doc = crate::doc_!(modules: crate::error; option)]
#![doc = crate::doc_!(newline)]
//!
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
