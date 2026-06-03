// devela::work::exec::thread
//
#![doc = crate::_DOC_WORK_EXEC_THREAD!()] // public
#![doc = crate::_doc!(modules: crate::work::exec; thread)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: thread)]
//

mod sleep; // sleep4!

#[cfg(feature = "std")]
crate::items! {
    mod _reexport_std;

    mod ext; // ThreadExt

    // mod local; // ThreadLocal WIP
    // mod manager_2; // ThreadManager, ThreadWork WIP
    // mod pool; // WIP
    // mod semaphore; // Semaphore WIP
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            sleep::*,
        };
        #[cfg(feature = "std")]
        pub use super::{
            ext::*,
            // local::*;
            // manager_2::*;
            // pool::*;
            // semaphore::*;
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
