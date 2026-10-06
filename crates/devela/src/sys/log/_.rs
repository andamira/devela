//
#![doc = crate::_DOC_SYS_LOG!()] // public
#![doc = crate::_doc!(modules: crate::sys; log)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod bench; //
    mod diag; // DiagLevel, DiagOut
    // mod logger; // LogLevel, Logger, log_with WIP
    mod slog; // LoggerStatic, slog!
    // mod trace; //
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            diag::*,
            // logger::*, // WIP
            slog::{LoggerStatic, slog},
        };
    }
}
