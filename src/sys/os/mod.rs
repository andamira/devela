// devela::sys::os
//
//! OS-specific.
#![doc = crate::doc_!(modules: crate::sys; os: linux)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: os)]
//

#[cfg(feature = "linux")]
crate::items! {
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
    pub mod linux;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use linux::all::*;
}

pub(crate) mod all {
    #[cfg(feature = "linux")]
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use super::linux::all::*;
}
