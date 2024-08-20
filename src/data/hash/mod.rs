// devela::data::hash
//
//! Generic hashing support.
#![doc = crate::code::doc_extends!(hash)]
//!
//

mod fnv;
mod fx;
mod pengy;
mod reexports;
#[allow(unused_imports)]
pub use {fnv::*, fx::*, pengy::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{fnv::*, fx::*, reexports::*};
}
