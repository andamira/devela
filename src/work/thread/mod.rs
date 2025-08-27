// devela::work::thread
//
#![doc = crate::_DOC_WORK_THREAD!()]
//!
#![doc = crate::_doc!(extends: thread)]
//

crate::mod_path!(std _s "../../../libs/base_std/src/work/thread/reexports.rs");

#[cfg(feature = "std")]
crate::items! {
    mod ext; // ExtThread
    mod sleep; // sleep4!
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::{_s::*, ext::*, sleep::*};
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
        pub use super::_s::*;
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
