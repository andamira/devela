// devela/src/sys/service/email/_.rs
//
#![doc = crate::_DOC_SYS_SERVICE_EMAIL!()] // public
#![doc = crate::_doc!(modules: crate::sys::service; email)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
    // pub mod_ imap; // WIP The IMAP protocol
    // pub mod_ smtp; // WIP The SMTP protocol
    // pub mod_ pop; // WIP The POP protocol
}
crate::mods_out! { // _pub_mods
    _pub_mods {
        // pub use super::{
        //     imap::_all::*,
        //     smtp::_all::*,
        //     pop::_all::*,
        // };
    }
}
