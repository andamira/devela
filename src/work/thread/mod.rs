// devela::work::thread
//
//! Native threads.
#![doc = crate::code::doc_!(extends: thread)]
#![doc = crate::code::doc_!(modules: crate::work; thread)]
#![doc = crate::code::doc_!(newline)]
//!
//

mod reexports;
mod sleep;

#[allow(unused_imports)]
pub use {reexports::*, sleep::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{reexports::*, sleep::*};
}
