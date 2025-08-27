// devela::work::process
//
#![doc = crate::_DOC_WORK_PROCESS!()]
//!
#![doc = crate::_doc!(extends: process)]
//

mod reexports;

#[cfg(feature = "std")]
mod ext; // ExtProcess

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
        #[cfg(feature = "std")]
        pub use super::ext::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::{ext::*, reexports::*};
    }
}
