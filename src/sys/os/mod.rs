// devela::sys::os
//
//! OS-specific.
#![doc = crate::code::doc_!(extends: os)]
#![doc = crate::code::doc_!(modules: crate::sys; os: linux)]
#![doc = crate::code::doc_!(newline)]
//!
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
