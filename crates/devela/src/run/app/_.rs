//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_RUN_APP!()] // private
#![doc = crate::_doc!(modules: crate::run; app)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "control")]
    mod control;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg(feature = "control")]
        pub use super::control::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(all(feature = "alloc", root_sys))]
        pub use crate::sys::fs::app::AppInfo;
    }
}
