// devela/src/sys/service/_.rs
//
#![doc = crate::_DOC_SYS_SERVICE!()] // public
#![doc = crate::_doc!(modules: crate::sys; service: web)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
    // pub mod_ device;
    // pub mod_ email;
    // pub mod_ file;
    // pub mod_ message;
    // pub mod_ remote;
    pub mod_ web; // The HTTP protocol
}
crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            // device::_all::*,
            // email::_all::*,
            // file::_all::*,
            // message::_all::*,
            // remote::_all::*,
            web::_all::*,
        };
    }
    _crate_internals {
        pub use super::web::_crate_internals::*;
    }
}
