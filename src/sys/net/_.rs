// devela/src/sys/net/_.rs
//
#![doc = crate::_DOC_SYS_NET!()] // public
#![doc = crate::_doc!(modules: crate::sys; net: http)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
        mod _reexport_core;
        #[cfg(feature = "std")]
        mod _reexport_std;

    #[cfg(feature = "http")]
    pub mod_ http; // The HTTP protocol
        // mod imap; // WIP The IMAP protocol
        // mod ftp; // WIP The FTP protocol
        // mod smtp; // WIP The SMTP protocol
        // mod_ telegram; // WIP The Telegram protocol
}
crate::mods_out! { // _pubmods, _reexports, _crate_internals
    _pub_mods {
        #[cfg(feature = "http")]
        pub use super::http::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }

    _crate_internals {
        #[cfg(feature = "http")]
        pub use super::http::_crate_internals::*;
    }
}
