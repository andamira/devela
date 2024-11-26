// devela::code::any
//
//! Dynamic typing and reflection.
// #![doc = crate::doc_!(extends: any)]
// #![doc = crate::doc_!(modules: crate::code; any)]
// #![doc = crate::doc_!(newline)]
//

mod ext;
mod reexports;
#[allow(unused_imports)]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
