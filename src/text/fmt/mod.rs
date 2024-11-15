// devela::text::fmt
//
//! Formatting strings.
//!
#![doc = crate::doc_!(extends: fmt)]
//

mod buf;
mod num_to_str;
mod reexports;
#[allow(unused_imports)]
pub use {buf::*, num_to_str::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{buf::*, num_to_str::*, reexports::*};
}
