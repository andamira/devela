//
#![doc = crate::_DOC_RUN_REGIME!()] // public
#![doc = crate::_doc!(modules: crate::run; regime)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    mod info; // RunSystemInfo

    #[cfg(feature = "cap")]
    mod_ cap; // RunCap*
    #[cfg(feature = "cap")]
    mod service;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            info::*,
        };
        #[cfg(feature = "cap")]
        pub use super::{
            cap::_all::*,
            service::{RunService, RunServiceProbe},
        };
    }
}
