// devela::sys::log
//
#![doc = crate::_DOC_SYS_LOG!()]
//

#[cfg(feature = "dep_log")]
crate::items! {
    mod config; // LogConfig
    mod ext; // ExtLogger
    mod namespace; // Log
    mod reexports; // ::log::*
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        #[cfg(feature = "dep_log")]
        pub use super::{config::*, ext::*, namespace::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)] pub use super::_mods::*;
    }
}
