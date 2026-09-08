// devela/src/sys/net/inet/_.rs
//
#![doc = crate::_DOC_SYS_NET_INET!()] // public
#![doc = crate::_doc!(modules: crate::sys::net; inet)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
    mod_ ip;
    // mod_ lowpan;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            ip::_all::*,
            // lowpan::_all::*,
        };
    }
}
