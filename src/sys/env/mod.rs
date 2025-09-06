// devela::sys::env
//
#![doc = crate::_DOC_SYS_ENV!()]
//!
#![doc = crate::_doc!(extends: env)]
//

crate::mod_path!(_c "../../../libs/base_core/src/sys/env/reexports.rs");
crate::mod_path!(std _s "../../../libs/base_std/src/sys/env/reexports.rs");

mod arg;
mod namespace;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod app;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{_c::*, arg::_all::*, namespace::*};

        #[cfg(feature = "std")]
        pub use super::{_s::*, app::*};
    }
    _always {
        pub use super::_c::*;
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
}
