// devela::code::any
//
//! Dynamic typing and reflection.
#![doc = crate::code::doc_extends!(any)]
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
