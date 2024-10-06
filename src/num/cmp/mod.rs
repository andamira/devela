// devela::num::cmp
//
//! Comparing and ordering values.
#![doc = crate::code::doc_!(extends: cmp)]
#![doc = crate::code::doc_!(modules: crate::num; cmp)]
#![doc = crate::code::doc_!(newline)]
//!
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(_some_cmp)]
mod compare; // `Compare`
#[cfg(_some_cmp)]
pub use compare::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[cfg(_some_cmp)]
    pub use super::compare::*;
}
