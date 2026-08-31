// devela/src/run/regime/_.rs
//
#![doc = crate::_DOC_RUN_REGIME!()] // public
#![doc = crate::_doc!(modules: crate::run; regime)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    mod_ cap; // RunCap*
    mod info; // RunSystemInfo
    mod service; // RunService[Probe]
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cap::_all::*,
            info::*,
            service::*,
        };
    }
}
