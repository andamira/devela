// devela::data::id
//
//! Data identifiers.
#![doc = crate::code::doc_!(newline)]
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids
pub use {pin::*, seq::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{pin::*, seq::*};
}
