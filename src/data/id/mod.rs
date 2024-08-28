// devela::data::id
//
//! Data identifiers.
//

mod pin;
mod seq;

pub use {pin::*, seq::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{pin::*, seq::*};
}
