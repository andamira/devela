// devela::data::id
//
//! Data identifiers.
//

mod seq;

pub use seq::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::seq::*;
}
