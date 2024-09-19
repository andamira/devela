// devela::sys::os
//
//! OS-specific.
#![doc = crate::code::doc_extends!(os)]
//!
//

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
crate::items! {
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
