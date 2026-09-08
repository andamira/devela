// devela/src/sys/net/transport/tcp/_.rs
//
#![doc = crate::_DOC_SYS_NET_TRANSPORT_TCP!()] // private
#![doc = crate::_doc!(modules: crate::sys::net::transport; tcp)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
    #[cfg(feature = "std")]
    mod _reexport_std;

}
crate::mods_out! { // _mods, _reexports
    _mods {
        // pub use super::{
        //     __::_all::*,
        // };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
