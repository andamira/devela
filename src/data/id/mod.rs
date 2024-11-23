// devela::data::id
//
//! Data identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{pin::*, seq::*};
}
