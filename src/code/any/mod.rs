// devela::code::any
//
//! Dynamic typing and reflection.
#![doc = crate::code::doc_!(extends: any)]
#![doc = crate::code::doc_!(modules: crate::code; any)]
#![doc = crate::code::doc_!(newline)]
//!
//

mod ext;
mod reexports;
#[allow(unused_imports)]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
