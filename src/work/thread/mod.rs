// devela::work::thread
//
//! Native threads.
//!
#![doc = crate::doc_!(extends: thread)]
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
