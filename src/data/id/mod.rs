// devela::data::id
//
//! Data identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

pub(crate) mod all {
    #![allow(unused_imports)]
    #[doc(inline)]
    pub use super::{pin::*, seq::*};
}
