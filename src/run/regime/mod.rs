// devela::run::regime
//
#![doc = crate::_DOC_RUN_REGIME!()] // public
#![doc = crate::_doc!(modules: crate::run; regime)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

mod cap; // RunCap*
mod info; // RunSystemInfo
mod service; // RunService[Probe]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cap::*,
            info::*,
            service::*,
        };
    }
}
