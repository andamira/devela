// devela::work::thread
//
#![doc = crate::_DOC_WORK_THREAD!()] // public
#![doc = crate::_doc!(modules: crate::work; thread)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: thread)]
//

#[cfg(feature = "std")]
crate::items! {
    mod _reexport_std; // SYMLINK to /crates/base/std/src/work/thread/_reexport.rs

    mod ext; // ThreadExt
    mod sleep; // sleep4!

    // WIPZONE
    // mod local; // ThreadLocal
    // mod manager_2; // ThreadManager, ThreadWork
    // mod pool;
    // mod semaphore; // Semaphore
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            ext::*,
            sleep::*,

            // WIPZONE
            // local::*;
            // manager_2::*; // WIP
            // pool::*;
            // semaphore::*;
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
