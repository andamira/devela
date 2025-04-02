// devela::work::thread
//
//! Native threads.
//!
#![doc = crate::doc_!(extends: thread)]
//

#[cfg(feature = "std")]
crate::items! {
    mod ext; // ExtThread
    mod reexports;
    mod sleep; // sleep4!
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::{ext::*, reexports::*, sleep::*};
        // WIPZONE
        #[cfg(feature = "std")]
        crate::items! {
            // pub use super::local::*;
            // pub use super::manager_2::*; // WIP
            // pub use super::pool::*;
            // pub use super::semaphore::*;
        }
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::reexports::*;
    }
}
// WIPZONE
#[cfg(feature = "std")]
crate::items! {
    // mod local; // ThreadLocal
    // mod manager_2; // ThreadManager, ThreadWork
    // mod pool;
    // mod semaphore; // Semaphore
}
