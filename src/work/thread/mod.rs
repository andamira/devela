// devela::work::thread
//
#![doc = crate::_DOC_WORK_THREAD!()]
//!
#![doc = crate::_doc!(extends: thread)]
//

crate::mod_path!(std _s "../../../libs/base_std/src/work/thread/reexports.rs");

#[cfg(feature = "std")]
crate::items! {
    mod ext; // ThreadExt
    mod sleep; // sleep4!
}

// WIPZONE
#[cfg(feature = "std")]
crate::items! {
    // mod local; // ThreadLocal
    // mod manager_2; // ThreadManager, ThreadWork
    // mod pool;
    // mod semaphore; // Semaphore
}

crate::structural_mods! { // _mods
    _mods {
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
}
