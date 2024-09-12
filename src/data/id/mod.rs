// devela::data::id
//
//! Data identifiers.
//

mod pin; // pinned memory-based ids
mod res; // type-safe resource ids
mod seq; // static sequential ids
pub use {pin::*, res::*, seq::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{pin::*, res::*, seq::*};
}
