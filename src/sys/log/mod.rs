// devela::sys::log
//
#![doc = crate::_DOC_SYS_LOG!()] // public
#![doc = crate::_doc!(modules: crate::sys; log)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_dep; // ::log::*

// mod bench; //
mod diag; // DiagLevel, DiagOut
// mod logger; // LogLevel, Logger, log_with WIP
mod slog; // LoggerStatic, slog!
// mod trace; //

#[cfg(feature = "dep_log")]
crate::items! {
    mod config; // LogConfig
    mod ext; // LoggerExt
    mod namespace; // Log
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            diag::*,
            // logger::*, // WIP
            slog::*,
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
