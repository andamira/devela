// devela::work::process
//
//! Native processes.
//!
#![doc = crate::doc_!(extends: process)]
//

#[cfg(feature = "std")]
crate::items! {
    mod ext; // ExtProcess
    mod reexports;
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        #[cfg(feature = "std")]
        pub use super::{ext::*, reexports::*};
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
