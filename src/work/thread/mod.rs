// devela::work::thread
//
//! Native threads.
#![doc = crate::doc_!(extends: thread)]
#![doc = crate::doc_!(modules: crate::work; thread)]
#![doc = crate::doc_!(newline)]
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
