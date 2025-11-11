// devela::sys::log
//
#![doc = crate::_DOC_SYS_LOG!()]
//

#[cfg(feature = "dep_log")]
crate::items! {
    mod config; // LogConfig
    mod ext; // LoggerExt
    mod namespace; // Log
    mod reexports; // ::log::*
}

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "dep_log")]
        pub use super::{config::*, ext::*, namespace::*, reexports::*};

        #[doc(inline)]
        pub use devela_base_core::sys::log::{
            LoggerStatic, slog,
        };
    }
}
