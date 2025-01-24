// devela::work::process
//
//! Native processes and threads.
//!
#![doc = crate::doc_!(extends: process, thread)]
//

#[cfg(feature = "std")]
crate::items! {
    mod ext; // ExtProcess
    mod reexports;
    mod thread;
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        #[cfg(feature = "std")]
        pub use super::{ext::*, reexports::*, thread::_all::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::{ext::*, reexports::*, thread::_always::*};
    }
}
