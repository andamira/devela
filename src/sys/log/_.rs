// devela/src/sys/log/_.rs
//
#![doc = crate::_DOC_SYS_LOG!()] // public
#![doc = crate::_doc!(modules: crate::sys; log)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod _reexport_dep; // ::log::*

    // mod bench; //
    mod diag; // DiagLevel, DiagOut
    // mod logger; // LogLevel, Logger, log_with WIP
    mod slog; // LoggerStatic, slog!
    // mod trace; //

    #[cfg(feature = "dep_log")]
    mod config; // LogConfig
    #[cfg(feature = "dep_log")]
    mod ext; // LoggerExt
    #[cfg(feature = "dep_log")]
    mod namespace; // Log
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            diag::*,
            // logger::*, // WIP
            slog::{LoggerStatic, slog},
        };
        #[cfg(feature = "dep_log")]
        pub use super::{
            config::*,
            ext::*,
            namespace::*,
        };
    }
    _reexports {
        pub use super::_reexport_dep::*;
    }
}
