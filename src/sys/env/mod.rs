// devela::sys::env
//
#![doc = crate::_DOC_SYS_ENV!()]
//!
#![doc = crate::_doc!(extends: env)]
//

crate::mod_path!(_c "../../../libs/base/src/sys/env/reexports.rs");
crate::mod_path!(std _s "../../../libs/base_std/src/sys/env/reexports.rs");

mod arg;
mod namespace;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod app;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{_c::*, arg::_all::*, namespace::*};

        #[cfg(feature = "std")]
        pub use super::{_s::*, app::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_c::*;
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
}
