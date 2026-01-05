// devela::sys::log
//
#![doc = crate::_DOC_SYS_LOG!()]
//

mod _reexport_dep; // ::log::*

#[cfg(feature = "dep_log")]
crate::items! {
    mod config; // LogConfig
    mod ext; // LoggerExt
    mod namespace; // Log
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "dep_log")]
        pub use super::{
            config::*,
            ext::*,
            namespace::*,
        };
    }
    _reexports {
        pub use super::_reexport_dep::*;
        #[doc(inline)]
        pub use devela_base_core::sys::log::{
            LoggerStatic, slog,
        };
    }
}
