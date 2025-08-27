// devela::sys::os
//
//! OS-specific.
#![doc = crate::_doc!(modules: crate::sys; os: linux)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: os)]
//

mod print; // os_[e]print[ln]!

#[cfg(feature = "linux")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linux")))]
pub mod linux;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::print::*;
    }
    mod _pub_mods {
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
