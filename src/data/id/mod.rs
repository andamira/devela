// devela::data::id
//
//! Data identifiers.
//

mod marker; // zero-cost generic ids
mod pin; // pinned memory-based ids
mod res; // type-safe resource ids
mod seq; // static sequential ids
pub use {marker::*, pin::*, res::*, seq::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{marker::*, pin::*, res::*, seq::*};
}
