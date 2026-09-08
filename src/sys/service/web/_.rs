// devela/src/sys/service/web/_.rs
//
#![doc = crate::_DOC_SYS_SERVICE_WEB!()] // public
#![doc = crate::_doc!(modules: crate::sys::service; web: http)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "http")]
    pub mod_ http; // The HTTP protocol
}
crate::mods_out! { // _pubmods, _crate_internals
    _pub_mods {
        #[cfg(feature = "http")]
        pub use super::http::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "http")]
        pub use super::http::_crate_internals::*;
    }
}
