// devela::text::fmt
//
//! Formatting strings.
#![doc = crate::code::doc_extends!(fmt)]
//!
//

mod misc;
mod num_to_str;
mod reexports;
#[allow(unused_imports)]
pub use {misc::*, num_to_str::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{misc::*, num_to_str::*, reexports::*};
}
