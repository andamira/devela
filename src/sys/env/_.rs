// devela/src/sys/env/_.rs
//
#![doc = crate::_DOC_SYS_ENV!()] // public
#![doc = crate::_doc!(modules: crate::sys; env)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: env)]
//

crate::mods_in! {
    mod _reexport_core;
    #[cfg(feature = "std")]
    mod _reexport_std;

    mod_ arg;
    mod namespace;

    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod app;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            arg::_all::*,
            namespace::*,
        };

        #[cfg(feature = "std")]
        pub use super::app::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
