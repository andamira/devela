// devela/src/sys/service/file/_.rs
//
#![doc = crate::_DOC_SYS_SERVICE_FILE!()] // public
#![doc = crate::_doc!(modules: crate::sys::service; file)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
        // mod ftp; // WIP The FTP protocol
        // mod mtp; // WIP The MTP protocol
}
crate::mods_out! { // _pub_mods
    _pub_mods {
        pub use super::{
            ftp::_all::*,
            mtp::_all::*,
        };
    }
}
