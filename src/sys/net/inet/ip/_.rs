// devela/src/sys/net/inet/_.rs
//
#![doc = crate::_DOC_SYS_NET_INET!()] // public
#![doc = crate::_doc!(modules: crate::sys::net; inet)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
        mod _reexport_core;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        // pub use super::{
        // };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
